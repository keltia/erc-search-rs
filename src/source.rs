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
        let s = Source::new();

        let src = match cfg.sources.get(tag) {
            Some(s) => s.clone(),
            _ => bail!("Config not found"),
        };
        Ok(s)
    }

    /// Does the connection to the LDAP server
    pub fn connect(self: &Source) -> Result<LdapConn> {
        let url = format!("ldap://{}:{:?}/", self.site, self.port);
        Ok(LdapConn::new(&url)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = Source::new();
        assert!(
            Source {
                domain: "".to_string(),
                site: "".to_string(),
                port: Option::None,
                base: "".to_string(),
                filter: Option::None,
                attrs: Option::None,
            },
            s
        );
    }
}
