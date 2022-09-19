use std::process::Command;

use clap::Parser;

mod cli;
mod manifest;
use crate::cli::args::Args;
use crate::cli::commands::Commands;

fn main() {
    let cli = Args::parse();

    if let Some(config_file) = cli.config.as_deref() {
        println!(
            "Value for config: {}",
            config_file
                .to_str()
                .unwrap_or("COULD NOT PARSE PATH TO STR!")
        );
    }
    if cli.verbose {
        println!("Chose verbose!");
    }

    match &cli.command {
        Some(Commands::Sync) => {
            println!("Chose Sync")
        }
        None => {}
    };

    let output = Command::new("git")
        .args([
            "clone",
            "git@github.com:tbreslein/ansible.git",
            "/home/tommy/test/ansible",
        ])
        .output()
        .expect("guess it failed");
    println!("status: {}", output.status);
}
