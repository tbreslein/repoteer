use std::{
    env::{self, VarError},
    fs, io,
    path::PathBuf,
};

use self::repo::Repo;
use serde::Deserialize;
use toml;

#[derive(Deserialize, Debug, PartialEq)]
/// The record of which Repos should be managed by Repoteer
pub struct Manifest {
    /// Vector of the Repository declarations
    pub repos: Vec<Repo>,
}

#[derive(Debug)]
pub enum Error {
    TomlError(toml::de::Error),
    FSError(io::Error),
    VarError(VarError),
}

impl Manifest {
    /// Returns a `Result<manifest::Manifest, manifest::Error>` from an `Option<PathBuf>`
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
    pub fn new(opt_toml_path: Option<PathBuf>) -> Result<Self, Error> {
        return match opt_toml_path {
            Some(toml_path) => Self::from_toml_file(toml_path),
            None => {
                let home_path_str = env::var("HOME");
                if let Err(e) = home_path_str {
                    return Err(Error::VarError(e));
                }
                let default_manifest_path: PathBuf = [
                    home_path_str.unwrap(),
                    ".config".to_string(),
                    "repoteer".to_string(),
                    "manifest.toml".to_string(),
                ]
                .iter()
                .collect();

                Self::from_toml_file(PathBuf::from(default_manifest_path))
            }
        };
    }

    /// Returns a `Result<manifest::Manifest, manifest::Error>` from a `PathBuf`
    /// file
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
    fn from_toml_file(toml_path: PathBuf) -> Result<Self, Error> {
        return match fs::read_to_string(toml_path) {
            Ok(s) => Self::from_toml_str(s.as_str()),
            Err(e) => Err(Error::FSError(e)),
        };
    }

    /// Returns a `Result<manifest::Manifest, manifest::Error>` from a toml formatted string
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
    fn from_toml_str(toml_str: &str) -> Result<Self, Error> {
        return match toml::from_str(toml_str) {
            Ok(man) => Ok(man),
            Err(e) => Err(Error::TomlError(e)),
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
