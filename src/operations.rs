use std::{path::Path, process::Output};

use color_eyre::Result;

use crate::{
    cli::command::Command,
    manifest::{repo::Repo, Manifest},
};

pub fn run_operations(command: Command, manifest: Manifest) -> () {
    for repo in manifest.repos.iter() {
        println!("Repo: {:?}", repo.url);
        println!("   at {:?}", repo.path);
        process(match command {
            Command::Clone => run_clone(&repo, false),
            Command::Pull => run_pull(&repo),
            Command::Push => run_push(&repo),
            Command::Sync => run_sync(&repo),
        });
        println!("");
    }
    return;
}

fn process(result: Result<Output>) -> () {
    match result {
        Ok(output) => {
            if output.status.success() {
                println!("   Success! Output: {:?}", output.stdout);
            } else {
                println!("   Failure! Output: {:?}", output.stderr);
            }
        }
        Err(report) => {
            println!("   Error! Report: {:?}", report);
        }
    };
}

fn has_unstaged_changes(path: &str) -> Result<bool> {
    return Ok(std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(path)
        .output()?
        .stdout
        .is_empty());
}

fn get_output_lines(output: Output) -> Result<Vec<String>> {
    return Ok(String::from_utf8(output.stdout)?.lines().map(|x| x.to_string()).collect());
}

fn has_worktrees(path: &str) -> Result<bool> {
    let output = std::process::Command::new("git")
        .args(["worktree", "list"])
        .current_dir(path)
        .output()?;
    let output_lines: Vec<String> = get_output_lines(output)?;
    return Ok(output_lines.len() == 1 && output_lines[0].contains("(bare)"));
}

// WIP:
// fn get_branches(path: &str) -> Result<Vec<String>> {
//     let output = std::process::Command::new("git")
//         .args(["worktree", "list"])
//         .current_dir(path)
//         .output()?;
//     let output_lines: Vec<String> = get_output_lines(output)?;
//     return Ok(output_lines.iter().skip_while(|line| line.len() > 2).);
// }

fn run_clone(repo: &Repo, is_syncing: bool) -> Result<Output> {
    return if Path::new(&repo.path).join(".git").exists() {
        if !is_syncing {
            Ok(std::process::Command::new("echo")
                .arg("Clone already exists")
                .output()?)
        } else {
            Ok(std::process::Command::new("").output()?)
        }
    } else {
        Ok(std::process::Command::new("git")
            .args([
                "clone",
                &repo.url,
                &repo.path,
                if repo.is_bare.is_some() && repo.is_bare.unwrap() {
                    "--bare"
                } else {
                    ""
                },
            ])
            .output()?)
    };
}

fn run_pull(repo: &Repo) -> Result<Output> {
    let pull = |path, branch| std::process::Command::new("git")
            .args(["pull", "origin", branch])
            .current_dir(path)
            .output();

    let has_worktrees = has_worktrees(&repo.path)?;
    return if has_unstaged_changes(&repo.path)? {
        Ok(std::process::Command::new("echo")
            .arg("Has unstaged changes! Cancelling pull")
            .output()?)
    } else {
        Ok(pull(&repo.path, "master")?)
    };
}

fn run_push(repo: &Repo) -> Result<Output> {
    return Ok(std::process::Command::new("git")
        .args(["push"])
        .current_dir(&repo.path)
        .output()?);
}

fn run_sync(repo: &Repo) -> Result<Output> {
    return run_clone(repo, true)
        .and(run_pull(repo))
        .and(run_push(repo))
        .and(Ok(std::process::Command::new("echo")
            .arg("Sync complete!")
            .output()?));
}
