use clap::Subcommand;

#[derive(Subcommand, Debug, Copy, Clone)]
/// Enumerates the different commands you can pass to repoteer
pub enum Command {
    /// Clone (if repo is not cloned yet) or pull, then push repos
    Sync,

    /// Clone the repository, if it is not cloned yet
    Clone,

    /// Only pull remote change
    Pull,

    /// Only push local changes to remote
    Push,
}
