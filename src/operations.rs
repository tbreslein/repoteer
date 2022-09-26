use std::{path::Path, process::Output};

use color_eyre::{eyre::eyre, Result};

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

fn has_unstaged_changes(path: &str) -> Result<bool> {
    return Ok(!std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(path)
        .output()?
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

fn run_clone(repo: &Repo) -> Result<Output> {
    Ok(std::process::Command::new("git")
        // this is a bit ugly,  but unfortunately just setting the last arg to an empty string
        // in the case of passing --bare does not work, because the process still reads it as
        // an argument and then complains about receiving too many arguments.
        // basically, if I were to pass ["clone", &repo.url, &repo.path, ""], the command would
        // be `git clone <url> <dir> ""`, and then it would complain about that last "".
        .args(if repo.is_bare.is_some() && repo.is_bare.unwrap() {
            vec!["clone", &repo.url, &repo.path, "--bare"]
        } else {
            vec!["clone", &repo.url, &repo.path]
        })
        .output()?)
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

fn run_pull(repo: &Repo) -> Result<Output> {
    // TODO: this and push closure can be abstracted
    let pull = |path: &str, branch: &str| {
        Ok(std::process::Command::new("git")
            .args(["pull", "origin", branch])
            .current_dir(path)
            .output()?)
    };
    return if has_unstaged_changes(&repo.path)? {
        Err(eyre!(
            "Repo has unstaged changes on branch {} pull aborted!",
            get_current_branch(&repo.path)?
        ))
    } else {
        run_operation_with_worktrees(&repo, pull, "Pull")
    };
}

fn run_push(repo: &Repo) -> Result<Output> {
    let push = |path: &str, branch: &str| {
        return Ok(std::process::Command::new("git")
            .args(["push", "origin", branch])
            .current_dir(path)
            .output()?);
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
