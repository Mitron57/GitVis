use git2::{DiffOptions, Repository};
use std::collections::HashSet;
use std::fmt;

pub struct CommitNode {
    pub id: String,
    pub message: String,
    pub parents: Vec<String>,
}

impl fmt::Display for CommitNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for parent in &self.parents {
            writeln!(f, "{} -down-> {}", parent, self.id)?;
        }
        writeln!(
            f,
            "{} : \"{}\"",
            self.id,
            self.message.trim().replace("\n", " ")
        )
    }
}

pub struct CommitTree {
    pub file: String,
    pub nodes: Vec<CommitNode>,
}

impl CommitTree {
    pub fn to_plantuml_string(&self) -> String {
        let mut plantuml_string = String::new();
        plantuml_string.push_str("@startuml\n");
        let mut parents = HashSet::new();
        for node in &self.nodes {
            plantuml_string.push_str(format!("{}", node).as_str());
            parents.extend(node.parents.iter().cloned());
        }
        let leafs: Vec<&CommitNode> = self
            .nodes
            .iter()
            .filter(|&node| !parents.contains(node.id.as_str()))
            .collect();
        for leaf in leafs {
            plantuml_string.push_str(format!("{} -down-> \"{}\"\n", leaf.id, self.file).as_str());
        }
        plantuml_string.push_str("@enduml\n");
        plantuml_string
    }
}

pub struct Glob<'a> {
    repo: Repository,
    file: &'a str,
}

impl<'a> Glob<'a> {
    pub fn new(repo: Repository, file: &'a str) -> Glob<'a> {
        Glob { repo, file }
    }
}

impl TryFrom<Glob<'_>> for CommitTree {
    type Error = Box<dyn std::error::Error>;

    fn try_from(glob: Glob) -> Result<Self, Self::Error> {
        let mut revwalk = glob.repo.revwalk()?;
        revwalk.push_glob("*")?;
        let mut file_commits = Vec::new();
        for commit_id in revwalk {
            let commit_id = commit_id?;
            let commit = glob.repo.find_commit(commit_id)?;
            let commit_tree = commit.tree()?;
            let mut modified = false;
            for i in 0..commit.parent_count() {
                let parent_commit = commit.parent(i)?;
                let parent_tree = parent_commit.tree()?;
                let mut diff_opts = DiffOptions::new();
                diff_opts.pathspec(glob.file);
                let diff = glob.repo.diff_tree_to_tree(
                    Some(&parent_tree),
                    Some(&commit_tree),
                    Some(&mut diff_opts),
                )?;
                if diff.deltas().len() > 0 {
                    modified = true;
                    break;
                }
            }
            if modified {
                let commit_node = CommitNode {
                    id: commit.id().to_string(),
                    message: commit.message().unwrap_or("No commit message").to_string(),
                    parents: commit.parents().map(|p| p.id().to_string()).collect(),
                };
                file_commits.push(commit_node);
            }
        }
        file_commits.reverse();
        let mut filtered_commits = Vec::new();
        for node in &file_commits {
            let filtered_parents: Vec<String> = node
                .parents
                .iter()
                .filter(|parent_id| file_commits.iter().any(|c| &c.id == *parent_id))
                .cloned()
                .collect();
            filtered_commits.push(CommitNode {
                id: node.id.clone(),
                message: node.message.clone(),
                parents: filtered_parents,
            });
        }
        let tree = CommitTree {
            file: glob.file.to_string(),
            nodes: filtered_commits,
        };
        Ok(tree)
    }
}
