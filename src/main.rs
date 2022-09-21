use clap::Parser;
use color_eyre::eyre::Report;
use operations::run_operations;
use tracing::instrument;

mod cli;
mod config;
mod manifest;
mod operations;

#[instrument]
fn main() -> Result<(), Report> {
    #[cfg(feature = "capture-spantrace")]
    install_tracing();
    color_eyre::install()?;

    let (manifest, config, command) = {
        let cli = cli::args::Args::parse();
        (
            manifest::Manifest::new(&cli.manifest)?,
            config::Config::new(&cli),
            cli.command.unwrap_or(cli::command::Command::Sync)
        )
    };

    run_operations(command, manifest, config)?;

    return Ok(());
}

#[cfg(feature = "capture-spantrace")]
fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
