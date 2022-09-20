use crate::cli::args::Args;

pub struct Config {
    pub verbose: bool,
}

impl Config {
    pub fn new(args: &Args) -> Self {
        return Config {verbose: args.verbose};
    }
}
