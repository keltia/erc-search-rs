//! Everything related to a source of information

use anyhow::{anyhow, bail, Result};
use ldap3::{Ldap, LdapConn};
use serde::Deserialize;

use crate::config::Config;

/// Describe a source of information (aka LDAP-compatible server)
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Source {
    /// DNS domain used to search server addresses through SRV RRs
    pub domain: String,
    /// Server
    pub site: String,
    /// LDAP port (389, 636)
    pub port: Option<u16>,
    /// LDAP base
    pub base: String,
    /// Filter for LDAP queries
    pub filter: Option<String>,
    /// List of attributes we are interested in
    pub attrs: Option<Vec<String>>,
}

/// Source methods
impl Source {
    /// Creates an empty source.
    pub fn new() -> Source {
        Source {
            ..Default::default()
        }
    }

    /// Fetch a source from the configuration.
    pub fn from(cfg: &Config, tag: &str) -> Result<Self> {
        let src = match cfg.sources.get(tag) {
            Some(s) => s.clone(),
            _ => bail!("Config not found"),
        };
        Ok(src)
    }

    /// Does the connection to the LDAP server
    pub fn connect(&self) -> Result<LdapConn> {
        let url = format!("ldap://{}:{:?}/", self.site, self.port);
        Ok(LdapConn::new(&url)?)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_new() {
        let s = Source::new();
        assert_eq!(
            Source {
                domain: "".to_string(),
                site: "".to_string(),
                port: None,
                base: "".to_string(),
                filter: None,
                attrs: None,
            },
            s
        );
    }

    #[test]
    fn test_source_from() {
        let cn = PathBuf::from("bin/erc-search/config.toml");
        let cfg = Config::load(&cn);
        assert!(cfg.is_ok());

        let cfg = cfg.unwrap();
        let s = Source::from(&cfg, "people");
        assert!(s.is_ok());

        let s = s.unwrap();
        assert_eq!("ldap.eurocontrol.fr", s.site.as_str());
    }
}
