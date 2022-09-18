use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Reads from specific config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Verbose output
    #[clap(short, long, action)]
    verbose: bool,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Clone (if repo is not cloned yet) or pull, then push repos
    Sync,
}

fn main() {
    let cli = Args::parse();

    if let Some(config_file) = cli.config.as_deref() {
        println!("Value for config: {}", config_file.to_str().unwrap_or("COULD NOT PARSE PATH TO STR!"));
    }
    println!("Value for verbose: {}", cli.verbose);

    match &cli.command {
        Some(Commands::Sync) => {println!("Chose Sync")}
        None => {}
    }
}
