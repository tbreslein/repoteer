use self::repo::Repo;
use serde::Deserialize;
use toml;

#[derive(Deserialize, Debug, PartialEq)]
/// The record of which Repos should be managed by Repoteer
pub struct Manifest {
    /// Vector of the Repository declarations
    pub repos: Vec<Repo>,
}

impl Manifest {
    /// Returns a Result<Manifest, toml::de::Error> from a toml formatted string
    ///
    /// # Arguments
    ///
    /// * `s` - A toml formatted string
    ///
    /// # Examples
    ///
    /// ```
    /// let s = r#"
    ///     [[repos]]
    ///     url = "git@github.com:testuser/testrepo.git"
    ///     service = "Git"
    ///     path = "/home/foo/testrepo"

    ///     [[repos]]
    ///     url = "git@bitbucket.com:bbuser/somerepo.git"
    ///     service = "Git"
    ///     path = "/home/bar/somerepo"
    /// "#;
    /// let manifest = Manifest::from_toml_str(s);   
    /// ```
    pub fn from_toml_str(s: &str) -> Result<Manifest, toml::de::Error> {
        return toml::from_str(s);
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
    fn from_toml_str_empty_repos() {
        let s = r#"
            [[repos]]
        "#;
        assert!(Manifest::from_toml_str(s).is_err());
    }

    #[test]
    fn from_toml_str_empty_string() {
        let s = "";
        assert!(Manifest::from_toml_str(s).is_err());
    }
}
