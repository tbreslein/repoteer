use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Clone (if repo is not cloned yet) or pull, then push repos
    Sync,
}
