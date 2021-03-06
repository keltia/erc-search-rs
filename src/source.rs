//! Everything related to a source of information

use ldap3::{Ldap, LdapConn};
use serde::Deserialize;

use crate::config::Config;

/// Describe a source of information (aka LDAP-compatible server)
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Source {
    /// DNS domaine used to search server adresses through SRV RRs
    domain: String,
    /// Server
    site: String,
    /// LDAP port (389, 636)
    port: Option<u16>,
    /// LDAP base
    base: String,
    /// Filter for LDAP queries
    filter: Option<String>,
    /// List of attributes we are interested in
    attrs: Option<Vec<String>>,
}

/// Yield an empty source
impl Default for Source {
    fn default() -> Self {
        Self::new()
    }
}

/// Source methods
impl Source {
    /// Creates an empty source.
    pub fn new() -> Source {
        Source {
            domain: "".to_string(),
            site: "".to_string(),
            port: Option::None,
            base: "".to_string(),
            filter: Option::None,
            attrs: Option::None,
        }
    }

    /// Fetch a source from the configuration.
    pub fn from(cfg: Config, tag: &str) -> Source {
        let s = Source::new();

        let src = cfg.sources.get(tag);
        return match src {
            Some(src) => src.clone(),
            None => s,
        };
    }

    /// Does the connection to the LDAP server
    pub fn connect(self: &Source) -> Result<ldap3::LdapConn, ldap3::LdapError> {
        let url = format!("ldap://{}:{:?}/", self.site, self.port);

        let mut ldap = LdapConn::new(&url);
        match ldap {
            Ok(ldap) => Ok(ldap),
            Err(e) => Err(e),
        }
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
