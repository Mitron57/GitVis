use std::io::Write;
use std::process::Command;
use git_visual::{CommitNode, CommitTree, Config, Glob};

#[test]
fn test_config() -> Result<(), Box<dyn std::error::Error>> {
    let file = "header1,header2,header3,header4\njar,path,img,file\n";
    let filename= "tmp.csv";
    std::fs::File::create(filename)?.write_all(file.as_bytes())?;
    let config = Config::new_from_file(filename)?;
    assert_eq!(config.visualization_program, "jar");
    assert_eq!(config.repository_path, "path");
    assert_eq!(config.image_name, "img");
    assert_eq!(config.file_path, "file");
    std::fs::remove_file(filename)?;
    Ok(())
}

#[test]
fn test_plantuml() {
    let tree = CommitTree {
        file: "main.rs".into(),
        nodes: vec![CommitNode {
            id: "hash".into(),
            message: "Initial commit".into(),
            parents: vec![],
        }],
    };
    let plantuml =
        "@startuml\nhash : \"Initial commit\"\nhash -down-> \"main.rs\"\n@enduml\n".to_owned();
    assert_eq!(plantuml, tree.to_plantuml_string());
}

#[test]
fn test_git_repo() -> Result<(), Box<dyn std::error::Error>> {
    let repo_dir = "./tmp/";
    let file = "./tmp/main.cpp";
    if let Err(err) = std::fs::create_dir(repo_dir) {
        if err.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(err.into());
        }
    }
    Command::new("git").args(["init", repo_dir]).output()?;
    std::fs::File::create(file)?;
    Command::new("git")
        .args(["add", "main.cpp"])
        .current_dir(repo_dir)
        .output()?;
    Command::new("git")
        .args(["commit", "-m", "test1"])
        .current_dir(repo_dir)
        .output()?;
    std::fs::write(file, "std::cout << \"Hello, World!\"")?;
    Command::new("git")
        .args(["add", "main.cpp"])
        .current_dir(repo_dir)
        .output()?;
    Command::new("git")
        .args(["commit", "-m", "test2"])
        .current_dir(repo_dir)
        .output()?;
    let mock = git2::Repository::open(repo_dir)?;
    let glob = Glob::new(mock, "main.cpp");
    let tree = CommitTree::try_from(glob)?;
    assert_eq!(tree.nodes.len(), 1);
    let node = tree.nodes.first().unwrap();
    assert!(node.parents.is_empty());
    assert_eq!(node.message.trim(), "test2");
    std::fs::remove_dir_all(repo_dir)?;
    Ok(())
}
