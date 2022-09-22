use std::path::Path;

use color_eyre::{Report, Result};

use crate::{
    cli::command::Command,
    config::Config,
    manifest::{repo::Repo, Manifest},
};

pub fn run_operations(command: Command, manifest: Manifest, _config: Config) -> Result<(), Report> {
    for repo in manifest.repos.iter() {
        println!("Repo: {:?}", repo.url);
        println!("   at {:?}", repo.path);
        let output = match command {
            Command::Clone => run_clone(&repo, false)?,
            Command::Pull => run_pull(&repo)?,
            Command::Push => run_push(&repo)?,
            Command::Sync => run_sync(&repo)?,
        };
        if output.status.success() {
            println!("   Success! Output: {:?}", output.stdout);
        } else {
            println!("   Failure! Output: {:?}", output.stderr);
        }
        println!("");
    }
    return Ok(());
}

fn has_unstaged_changes(path: &str) -> Result<bool, Report> {
    return Ok(std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(path)
        .output()?.stdout.is_empty());
}

fn run_clone(repo: &Repo, is_syncing: bool) -> Result<std::process::Output, Report> {
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
            .args(["clone", &repo.url, &repo.path])
            .output()?)
    };
}

fn run_sync(repo: &Repo) -> Result<std::process::Output, Report> {
    run_clone(repo, true)?;
    run_pull(repo)?;
    run_push(repo)?;
    return Ok(std::process::Command::new("echo").arg("Sync complete!").output()?);

}

fn run_pull(repo: &Repo) -> Result<std::process::Output, Report> {
    return if has_unstaged_changes(&repo.path)? {
        Ok(std::process::Command::new("echo")
            .arg("Has unstaged changes! Cancelling pull")
            .output()?)
    } else {
        Ok(std::process::Command::new("git")
            .args(["pull"])
            .current_dir(&repo.path)
            .output()?)
    };
}

fn run_push(repo: &Repo) -> Result<std::process::Output, Report> {
    return Ok(std::process::Command::new("git")
        .args(["push"])
        .current_dir(&repo.path)
        .output()?);
}
