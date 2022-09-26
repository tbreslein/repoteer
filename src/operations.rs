use std::{path::Path, process::Output};

use color_eyre::{eyre::bail, Result};

use crate::{
    cli::command::Command,
    manifest::{repo::Repo, Manifest},
};

/// Runs the operation given throught the CLI `command` field
///
/// # Arguments
///
/// * `command` - The `Command` the user gave when calling `repoteer`
/// * `manifest` - The `Manifest` holding info about the repositories being managed
///
/// # Examples
///
/// ```
/// let command = Command::clone;
/// let manifest = Manifest { ... };
/// run_operations(command, manifest);
/// ```
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

/// Enumerates the different git commands used throughout this module
///
///  NOTE: Yes, this has overlap with crate::cli::Command. No, I do not care because I want to limit
///  the repoteer cli commands and do not want to add things like StatusPorcelain to that list.
enum GitCommand {
    Clone,
    Pull,
    Push,
    StatusPorcelain,
}

impl GitCommand {
    /// Runs the git command declared by Self and returns a `eyre::Result<Output>`
    ///
    /// # Arguments
    ///
    /// * `self` - The `GitCommand` that called this method
    /// * `repo` - The `Repo` being operated on
    /// * `path` - The `path` where the command is being run
    /// * `branch` - The branch being operated on
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

/// Runs a `git clone` operation, defined in GitCommand::run(...) and returns a `eyre::Result<Output>`
///
/// # Arguments
///
/// * `repo` - The `Repo` being operated on
fn run_clone(repo: &Repo) -> Result<Output> {
    return GitCommand::Clone.run(&repo, &repo.path, "");
}

/// Runs a `git pull` operation, defined in GitCommand::run(...) and returns a `eyre::Result<Output>`
///
/// # Arguments
///
/// * `repo` - The `Repo` being operated on
fn run_pull(repo: &Repo) -> Result<Output> {
    let pull = |path: &str, branch: &str| {
        return GitCommand::Pull.run(&repo, &path, &branch);
    };
    return run_operation_with_worktrees(&repo, pull, "Pull");
}

/// Runs a `git push` operation, defined in GitCommand::run(...) and returns a `eyre::Result<Output>`
///
/// # Arguments
///
/// * `repo` - The `Repo` being operated on
fn run_push(repo: &Repo) -> Result<Output> {
    let push = |path: &str, branch: &str| {
        return GitCommand::Push.run(&repo, &path, &branch);
    };
    return run_operation_with_worktrees(&repo, push, "Push");
}

/// Runs a `run_clone`, in case the repository has not been cloned yet, otherwise it runs `run_pull` and `run_push`, and returns a `eyre::Result<Output>` in either way
///
/// # Arguments
///
/// * `repo` - The `Repo` being operated on
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

/// Wrapper function for running processing a `eyre::Result<Output>` and printing to stdout
///
/// # Arguments
///
/// * `result` - The `Result<Output>` being processed
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

/// Checks whether the branch at `path` has unstaged changes and returns a `eyre::Result<bool>`
///
/// This is useful for doing `git pull` commands, where the operation needs to error out in that
/// case.
///
/// # Arguments
///
/// * `repo` - Basically a dead argument, this is unfortunately needed for the GitCommand::run()
/// method...
/// * `path` - The path to the branch being checked
fn has_unstaged_changes(repo: &Repo, path: &str) -> Result<bool> {
    return Ok(!GitCommand::StatusPorcelain
        .run(&repo, &path, "")?
        .stdout
        .is_empty());
}

/// Parse an `Output.stdout` into a `Result<Vec<String>>` containing the lines out that stdout
///
/// # Arguments
///
/// * `output` - The `Output` being processed
fn get_output_lines(output: Output) -> Result<Vec<String>> {
    return Ok(String::from_utf8(output.stdout)?
        .lines()
        .map(|x| x.to_string())
        .collect());
}

/// Checks whether the repository at `path` is a worktree repository
///
/// # Arguments
///
/// * `path` - The path to the branch being checked
fn has_worktrees(path: &str) -> Result<bool> {
    let output = std::process::Command::new("git")
        .args(["worktree", "list"])
        .current_dir(path)
        .output()?;
    let output_lines: Vec<String> = get_output_lines(output)?;
    return Ok(output_lines.len() >= 1 && output_lines[0].contains("(bare)"));
}

/// Checks the repository at `path`, and returns a `Result<Vec<String>>` containing the different
/// branch names that have been checked out.
///
/// Only works for non-bare repositories.
///
/// # Arguments
///
/// * `path` - The path to the branch being checked
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

/// Checks the repository at `path`, and returns a `Result<Vec<String>>` containing the different
/// worktree names that have been checked out.
///
/// # Arguments
///
/// * `path` - The path to the branch being checked
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

/// Checks the repository at `path` and returns a `Result<String>` containing the name of the
/// current branch
///
/// # Arguments
///
/// * `path` - The path to the branch being checked
fn get_current_branch(path: &str) -> Result<String> {
    return Ok(String::from_utf8(
        std::process::Command::new("git")
            .args(["branch", "--show-current"])
            .current_dir(path)
            .output()?
            .stdout,
    )?);
}

/// Wrapper function for git operations where the semantics of the git commands change depending on
/// whether the local repository is bare / has worktrees or not
///
/// # Arguments
///
/// * `repo` - The `Repo` being processed
/// * `f` - The function being run
/// * `op` - Name of the operation, needed for terminal output
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
