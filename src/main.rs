use clap::Parser;
use color_eyre::eyre::Report;
use tracing::instrument;

mod cli;
mod manifest;

#[instrument]
fn main() -> Result<(), Report> {
    #[cfg(feature = "capture-spantrace")]
    install_tracing();

    color_eyre::install()?;

    let cli = cli::args::Args::parse();

    let manifest = manifest::Manifest::new(cli.manifest)?;

    return Ok(());

    // if let Some(config_file) = cli.config.as_deref() {
    //     println!(
    //         "Value for config: {}",
    //         config_file
    //             .to_str()
    //             .unwrap_or("COULD NOT PARSE PATH TO STR!")
    //     );
    // }
    // if cli.verbose {
    //     println!("Chose verbose!");
    // }

    // match &cli.command {
    //     Some(Commands::Sync) => {
    //         println!("Chose Sync")
    //     }
    //     None => {}
    // };

    // let output = Command::new("git")
    //     .args([
    //         "clone",
    //         "git@github.com:tbreslein/ansible.git",
    //         "/home/tommy/test/ansible",
    //     ])
    //     .output()
    //     .expect("guess it failed");
    // println!("status: {}", output.status);
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
