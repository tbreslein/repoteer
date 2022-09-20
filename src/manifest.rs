use std::{env, fs, path::PathBuf};

use self::repo::Repo;
use color_eyre::eyre::{eyre, Report};
use serde::Deserialize;
use toml;

#[derive(Deserialize, Debug, PartialEq)]
/// The record of which Repos should be managed by Repoteer
pub struct Manifest {
    /// Vector of the Repository declarations
    pub repos: Vec<Repo>,
}

impl Manifest {
    /// Returns a `Result<manifest::Manifest, Report>` from an `Option<PathBuf>`
    ///
    /// # Arguments
    ///
    /// * `toml_path` - string slice containing the path to a toml file
    ///
    /// # Examples
    ///
    /// ```
    /// let manifest = Manifest::from_toml_file("/path/to/some/toml/file.toml");   
    /// ```
    pub fn new(opt_toml_path: Option<PathBuf>) -> Result<Self, Report> {
        return match opt_toml_path {
            Some(toml_path) => Self::from_toml_file(toml_path),
            None => match env::var("HOME") {
                Ok(home_path_str) => {
                    return Self::from_toml_file(PathBuf::from(
                        [
                            home_path_str,
                            "/.config".to_string(),
                            "/repoteer".to_string(),
                            "/manifest.toml".to_string(),
                        ]
                        .concat(),
                    ));
                }
                e @ Err(_) => Err(eyre!("Unable to read env var HOME! Error: {:?}", e)),
            },
        };
    }

    /// Returns a `Result<manifest::Manifest, Report>` from a `PathBuf`
    /// file
    ///
    /// # Arguments
    ///
    /// * `toml_path` - `PathBuf` pointing to the manifest file
    ///
    /// # Examples
    ///
    /// ```
    /// let manifest = Manifest::from_toml_file(PathBuf::from("/path/to/some/toml/file.toml"));
    /// ```
    fn from_toml_file(toml_path: PathBuf) -> Result<Manifest, Report> {
        return match fs::read_to_string(&toml_path) {
            Ok(s) => Self::from_toml_str(s.as_str()),
            e @ Err(_) => Err(eyre!(
                "Unable to read from file {:?}! Error: {:?}",
                toml_path,
                e
            )),
        };
    }

    /// Returns a `Result<manifest::Manifest, Report>` from a toml formatted string
    ///
    /// # Arguments
    ///
    /// * `toml_str` - A toml formatted string
    ///
    /// # Examples
    ///
    /// ```
    /// let s = r#"
    ///     [[repos]]
    ///     url = "git@github.com:testuser/testrepo.git"
    ///     service = "Git"
    ///     path = "/home/foo/testrepo"
    ///
    ///     [[repos]]
    ///     url = "git@bitbucket.com:bbuser/somerepo.git"
    ///     service = "Git"
    ///     path = "/home/bar/somerepo"
    /// "#;
    /// let manifest = Manifest::from_toml_str(s);   
    /// ```
    fn from_toml_str(toml_str: &str) -> Result<Manifest, Report> {
        return match toml::from_str(toml_str) {
            Ok(man) => Ok(man),
            e @ Err(_) => Err(eyre!(
                "Unable to parse toml string to Manifesto instance! Error: {:?}",
                e
            )),
        };
    }
}

mod repo {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    /// Enumerates the types of repository services
    pub enum VCService {
        Git,
    }

    /// Models a single repository declaration
    #[derive(Deserialize, Debug, PartialEq)]
    pub struct Repo {
        /// URL of the remote repository
        pub url: String,

        /// The version control service that is used
        pub service: VCService,

        /// Where the repository should be cloned to on the local filesystem
        pub path: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_toml_str_single() {
        let s = r#"
            [[repos]]
            url = "git@github.com:testuser/testrepo.git"
            service = "Git"
            path = "/home/foo/testrepo"
        "#;
        let should_be = Manifest {
            repos: vec![Repo {
                url: "git@github.com:testuser/testrepo.git".to_string(),
                service: repo::VCService::Git,
                path: "/home/foo/testrepo".to_string(),
            }],
        };
        assert_eq!(Manifest::from_toml_str(s).unwrap(), should_be);
    }

    #[test]
    fn from_toml_file_single() {
        let path = PathBuf::from(r"test/tomlfiles/singlerepo.toml");
        let should_be = Manifest {
            repos: vec![Repo {
                url: "git@github.com:testuser/testrepo.git".to_string(),
                service: repo::VCService::Git,
                path: "/home/foo/testrepo".to_string(),
            }],
        };
        assert_eq!(Manifest::from_toml_file(path).unwrap(), should_be);
    }

    #[test]
    fn from_toml_str_multi() {
        let s = r#"
            [[repos]]
            url = "git@github.com:testuser/testrepo.git"
            service = "Git"
            path = "/home/foo/testrepo"

            [[repos]]
            url = "git@bitbucket.com:bbuser/somerepo.git"
            service = "Git"
            path = "/home/bar/somerepo"

            [[repos]]
            url = "git@gitlab.com:gitlabuser/gitlabrepo.git"
            service = "Git"
            path = "/root/gitlabrepo"
        "#;
        let should_be = Manifest {
            repos: vec![
                Repo {
                    url: "git@github.com:testuser/testrepo.git".to_string(),
                    service: repo::VCService::Git,
                    path: "/home/foo/testrepo".to_string(),
                },
                Repo {
                    url: "git@bitbucket.com:bbuser/somerepo.git".to_string(),
                    service: repo::VCService::Git,
                    path: "/home/bar/somerepo".to_string(),
                },
                Repo {
                    url: "git@gitlab.com:gitlabuser/gitlabrepo.git".to_string(),
                    service: repo::VCService::Git,
                    path: "/root/gitlabrepo".to_string(),
                },
            ],
        };
        assert_eq!(Manifest::from_toml_str(s).unwrap(), should_be);
    }

    #[test]
    fn from_toml_file_multi() {
        let path = PathBuf::from(r"test/tomlfiles/multirepo.toml");
        let should_be = Manifest {
            repos: vec![
                Repo {
                    url: "git@github.com:testuser/testrepo.git".to_string(),
                    service: repo::VCService::Git,
                    path: "/home/foo/testrepo".to_string(),
                },
                Repo {
                    url: "git@bitbucket.com:bbuser/somerepo.git".to_string(),
                    service: repo::VCService::Git,
                    path: "/home/bar/somerepo".to_string(),
                },
                Repo {
                    url: "git@gitlab.com:gitlabuser/gitlabrepo.git".to_string(),
                    service: repo::VCService::Git,
                    path: "/root/gitlabrepo".to_string(),
                },
            ],
        };
        assert_eq!(Manifest::from_toml_file(path).unwrap(), should_be);
    }

    #[test]
    fn from_toml_str_empty_repos() {
        let s = r#"
            [[repos]]
        "#;
        assert!(Manifest::from_toml_str(s).is_err());
    }

    #[test]
    fn from_toml_file_empty_repos() {
        let path = PathBuf::from(r"test/tomlfiles/emptyrepo.toml");
        assert!(Manifest::from_toml_file(path).is_err());
    }

    #[test]
    fn from_toml_str_empty_string() {
        let s = "";
        assert!(Manifest::from_toml_str(s).is_err());
    }

    #[test]
    fn from_toml_file_empty_string() {
        let path = PathBuf::from(r"test/tomlfiles/emptyfile.toml");
        assert!(Manifest::from_toml_file(path).is_err());
    }
}
