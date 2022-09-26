use std::{path::Path, process::Output};

use color_eyre::{eyre::bail, Result};

use crate::{
    cli::command::Command,
    manifest::{repo::Repo, Manifest},
};

pub fn run_operations(command: Command, manifest: Manifest) -> () {
    for repo in manifest.repos.iter() {
        println!("Repo:  {}", repo.url);
        println!("    at {}", repo.path);
        process(match command {
            Command::Clone => run_clone(&repo),
            Command::Pull => run_pull(&repo),
            Command::Push => run_push(&repo),
            Command::Sync => run_sync(&repo),
        });
        println!("");
    }
    return;
}

// NOTE: Yes, this has overlap with crate::cli::Command. No, I do not care because I want to limit
// the repoteer cli commands and do not want to add things like StatusPorcelain to that list.
enum GitCommand {
    Clone,
    Pull,
    Push,
    StatusPorcelain,
}

impl GitCommand {
    fn run(&self, repo: &Repo, path: &str, branch: &str) -> Result<Output> {
        let mut git_command_stump = std::process::Command::new("git");
        return Ok(match self {
            GitCommand::Clone => git_command_stump
                // this is a bit ugly,  but unfortunately just setting the last arg to an empty string
                // in the case of passing --bare does not work, because the process still reads it as
                // an argument and then complains about receiving too many arguments.
                // basically, if I were to pass ["clone", &repo.url, &repo.path, ""], the command would
                // be `git clone <url> <dir> ""`, and then it would complain about that last "".
                .args(if repo.is_bare.is_some() && repo.is_bare.unwrap() {
                    vec!["clone", &repo.url, &repo.path, "--bare"]
                } else {
                    vec!["clone", &repo.url, &repo.path]
                }),
            GitCommand::Pull => {
                if has_unstaged_changes(&repo, &repo.path)? {
                    bail!(
                        "Repo has unstaged changes on branch {} pull aborted!",
                        get_current_branch(&repo.path)?
                    );
                } else {
                    git_command_stump
                        .args(["pull", "origin", branch])
                        .current_dir(path)
                }
            }
            GitCommand::Push => git_command_stump
                .args(["push", "origin", branch])
                .current_dir(path),
            GitCommand::StatusPorcelain => git_command_stump
                .args(["status", "--porcelain"])
                .current_dir(path),
        }
        .output()?);
    }
}

fn run_clone(repo: &Repo) -> Result<Output> {
    return GitCommand::Clone.run(&repo, &repo.path, "");
}

fn run_pull(repo: &Repo) -> Result<Output> {
    let pull = |path: &str, branch: &str| {
        return GitCommand::Pull.run(&repo, &path, &branch);
    };
    return run_operation_with_worktrees(&repo, pull, "Pull");
}

fn run_push(repo: &Repo) -> Result<Output> {
    let push = |path: &str, branch: &str| {
        return GitCommand::Push.run(&repo, &path, &branch);
    };
    return run_operation_with_worktrees(&repo, push, "Push");
}

fn run_sync(repo: &Repo) -> Result<Output> {
    if !Path::new(&format!("{}/.git", repo.path)).exists() {
        run_clone(repo)?;
    } else {
        run_pull(repo)?;
        run_push(repo)?;
    }
    return Ok(std::process::Command::new("echo")
        .arg("Sync complete!")
        .output()?);
}

fn process(result: Result<Output>) -> () {
    match result {
        Ok(output) => {
            if output.status.success() {
                print!("    Success!");
                match std::str::from_utf8(&output.stdout) {
                    Ok(stdout) => {
                        if !stdout.is_empty() {
                            println!(" Output: {}", stdout);
                        }
                    }
                    Err(e) => {
                        println!(" Unable to convert output to String! Err: {:?}", e);
                    }
                }
            } else {
                println!(
                    "    Failure! Output: {}",
                    std::str::from_utf8(&output.stderr).unwrap_or("unknown error")
                );
            }
        }
        Err(report) => {
            println!("   Error! Report: {:?}", report);
        }
    };
}

fn has_unstaged_changes(repo: &Repo, path: &str) -> Result<bool> {
    return Ok(!GitCommand::StatusPorcelain
        .run(&repo, &path, "")?
        .stdout
        .is_empty());
}

fn get_output_lines(output: Output) -> Result<Vec<String>> {
    return Ok(String::from_utf8(output.stdout)?
        .lines()
        .map(|x| x.to_string())
        .collect());
}

fn has_worktrees(path: &str) -> Result<bool> {
    let output = std::process::Command::new("git")
        .args(["worktree", "list"])
        .current_dir(path)
        .output()?;
    let output_lines: Vec<String> = get_output_lines(output)?;
    return Ok(output_lines.len() >= 1 && output_lines[0].contains("(bare)"));
}

fn get_branches(path: &str) -> Result<Vec<String>> {
    let output = std::process::Command::new("git")
        .arg("branch")
        .current_dir(path)
        .output()?;
    return Ok(get_output_lines(output)?
        .into_iter()
        .map(|mut line| line.split_off(2))
        .collect());
}

fn get_worktrees(path: &str) -> Result<Vec<String>> {
    let output = std::process::Command::new("git")
        .args(["worktree", "list"])
        .current_dir(path)
        .output()?;
    return Ok(get_output_lines(output)?
        .into_iter()
        .filter(|line| line.ends_with("]"))
        .map(|line| {
            line.split_whitespace()
                .filter(|word| word.starts_with("[") && word.ends_with("]"))
                .map(|word| word.strip_prefix("[").unwrap_or(word))
                .map(|word| word.strip_suffix("]").unwrap_or(word))
                .collect()
        })
        .collect());
}

fn get_current_branch(path: &str) -> Result<String> {
    return Ok(String::from_utf8(
        std::process::Command::new("git")
            .args(["branch", "--show-current"])
            .current_dir(path)
            .output()?
            .stdout,
    )?);
}

fn run_operation_with_worktrees<F>(repo: &Repo, f: F, op: &str) -> Result<Output>
where
    F: Fn(&str, &str) -> Result<Output>,
{
    let has_worktrees = has_worktrees(&repo.path)?;
    let branches = if has_worktrees {
        get_worktrees(&repo.path)?
    } else {
        get_branches(&repo.path)?
    };

    println!("      running op: {}", op);
    for branch in branches.into_iter() {
        println!("        branch: {}", branch);
        let path = if has_worktrees {
            format!("{}/{}", &repo.path, &branch)
        } else {
            format!("{}", &repo.path)
        };
        match f(&path, &branch) {
            Ok(_) => {}
            Err(e) => {
                println!("   Error! Report: {}", e);
            }
        };
    }
    return Ok(std::process::Command::new("echo")
        .arg(format!("{} complete!", op))
        .output()?);
}
