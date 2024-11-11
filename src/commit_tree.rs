use git2::Repository;
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
            writeln!(f, "{} --> {}", parent, self.id)?;
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
    pub nodes: Vec<CommitNode>,
}

impl CommitTree {
    pub fn to_plantuml_string(&self) -> String {
        let mut plantuml_string = String::new();
        plantuml_string.push_str("@startuml\n");
        let mut seen_nodes = HashSet::new();
        for node in &self.nodes {
            if seen_nodes.insert(&node.id) {
                plantuml_string.push_str(format!("{}", node).as_str());
            }
        }
        plantuml_string.push_str("@enduml\n");
        plantuml_string
    }
}

pub fn parse_commits(repo: &Repository) -> Result<CommitTree, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_glob("*")?;
    let mut nodes = Vec::new();

    for commit_id in revwalk {
        let commit_id = commit_id?;
        let commit = repo.find_commit(commit_id)?;

        let commit_node = CommitNode {
            id: commit.id().to_string(),
            message: commit.summary().unwrap_or("No commit message").to_string(),
            parents: commit.parents().map(|p| p.id().to_string()).collect(),
        };
        nodes.push(commit_node);
    }
    nodes.reverse();
    Ok(CommitTree { nodes })
}
