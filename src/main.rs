use crate::args::Args;
use crate::commit_tree::{CommitTree, Glob};
use crate::config::Config;
use clap::Parser;
use git2::Repository;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::Command;

mod args;
mod commit_tree;
mod config;

fn launch_plantuml(script: String, config: Config) -> Result<(), Box<dyn Error>> {
    const TMP_GRAPH: &str = "tmp.puml";
    File::create(TMP_GRAPH)?.write_all(script.as_bytes())?;
    let _ = Command::new("java")
        .args(["-jar", &config.visualization_program, "-DPLANTUML_LIMIT_SIZE=10000",  TMP_GRAPH])
        .output()?;
    std::fs::rename("tmp.png", config.image_name)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let config = Config::new_from_file(&args.config_path)?;
    let repo = Repository::open(&config.repository_path)?;
    let glob = Glob::new(repo, &config.file_path);
    let tree = CommitTree::try_from(glob)?;
    let plantuml = tree.to_plantuml_string();
    launch_plantuml(plantuml, config)
}
