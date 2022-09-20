use crate::cli::args::Args;

/// Repoteer configuration
pub struct Config {
    /// Whether or not to print verbose output
    pub verbose: bool,
}

impl Config {
    /// Constructs a new `Config` from an `&Args`
    ///
    /// # Arguments
    ///
    /// * `args` - a &Args carrying CLI config
    ///
    /// # Examples
    ///
    /// ```
    /// let config = config::Config::new(&args);   
    /// ```
    pub fn new(args: &Args) -> Self {
        return Config {verbose: args.verbose};
    }
}
