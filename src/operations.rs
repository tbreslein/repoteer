use color_eyre::{Report, Result, eyre::Context};

use crate::{cli::command::Command, config::Config, manifest::{Manifest, repo::Repo}};

pub fn run_operations(command: Command, manifest: Manifest, _config: Config) -> Result<(), Report> {
    for repo in manifest.repos.iter() {
        println!("Repo: {:?}", repo.url);
        println!("   at {:?}", repo.path);
        let output = match command {
            Command::Clone => {run_clone(&repo)?}
            Command::Pull => {run_pull(&repo)?}
            Command::Push => {run_push(&repo)?}
            Command::Sync => {run_sync(&repo)?}
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

fn run_clone(repo: &Repo) -> Result<std::process::Output, Report> {
    return Ok(std::process::Command::new("git").args(["clone", &repo.url, &repo.path]).output()?);
}

fn run_sync(repo: &Repo) -> Result<std::process::Output, Report> {
    let output = std::process::Command::new("git").args(["clone"]).current_dir(&repo.path).output()?;
    return Ok(output);
}

fn run_pull(repo: &Repo) -> Result<std::process::Output, Report> {
    return Ok(std::process::Command::new("git").args(["pull"]).current_dir(&repo.path).output()?);
}

fn run_push(repo: &Repo) -> Result<std::process::Output, Report> {
    return Ok(std::process::Command::new("git").args(["push"]).current_dir(&repo.path).output()?);
}
