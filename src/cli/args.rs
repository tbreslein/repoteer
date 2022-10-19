use std::path::PathBuf;

use clap::Parser;

use super::command::Command;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Reads from specific manifest file.
    /// Defaults to $XDG_CONFIG_DIR/repoteer/manifest.toml
    #[clap(short, long, value_parser, value_name = "FILE")]
    pub manifest: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    return Args::command().debug_assert();
}
