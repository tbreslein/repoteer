use clap::Parser;
use cli::command::Command;
use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use colored::*;
use operations::run_operations;
use tracing::instrument;

mod cli;
mod manifest;
mod operations;

#[instrument]
#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "capture-spantrace")]
    install_tracing();
    color_eyre::install()?;

    let (manifest, command) = {
        let cli = cli::args::Args::parse();
        (
            manifest::Manifest::new(&cli.manifest)?,
            cli.command.unwrap_or(cli::command::Command::Sync),
        )
    };

    print_header(&command);
    run_operations(command, manifest).await
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
fn print_header(command: &Command) {
    println!(
        "{} {}
Copyright (c) 2022  Tommy Breslein <github.com/tbreslein>

Running command: {:?}\n",
        "repoteer".green(),
        VERSION,
        command.magenta()
    );
}
