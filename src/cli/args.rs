use std::path::PathBuf;

use clap::Parser;

use super::commands::Commands;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Reads from specific config file.
    /// Defaults to $XDG_CONFIG_DIR/repoteer/config.toml
    #[clap(short, long, value_parser, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Reads from specific manifest file.
    /// Defaults to $XDG_CONFIG_DIR/repoteer/manifest.toml
    #[clap(short, long, value_parser, value_name = "FILE")]
    pub manifest: Option<PathBuf>,

    /// Verbose output
    #[clap(short, long, action)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}
