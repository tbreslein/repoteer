use clap::Parser;
use color_eyre::eyre::Result;
use operations::run_operations;
use tracing::instrument;

mod cli;
mod manifest;
mod operations;

#[instrument]
fn main() -> Result<()> {
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

    {
        run_operations(command, manifest);
        Ok(())
    }
}
