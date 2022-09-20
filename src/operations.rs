use color_eyre::{Report, Result};

use crate::{manifest::Manifest, config::Config, cli::command::Command};

pub fn run_operations(command: Command, manifest: Manifest, config: Config) -> Result<(), Report> {
    Ok(())
}
