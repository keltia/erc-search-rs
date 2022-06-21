use anyhow::{anyhow, Result};
use ldap3::{LdapConn, LdapError, LdapResult, ResultEntry, Scope, SearchEntry};
use serde::Deserialize;

use crate::config::Config;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum Search {
    Machine,
    Person,
}

#[derive(Debug)]
pub struct Session {
    /// Empty config is an error.
    pub cfg: Option<Config>,
}

impl Default for Session {
    fn default() -> Self {
        Session { cfg: None }
    }
}

impl Session {
    pub fn search(self, what: String) -> Result<Vec<ResultEntry>> {
        if let cfg = self.cfg.clone().unwrap() {
            let src = match &cfg.sfor {
                Search::Person => &cfg.sources["person"].clone(),
                Search::Machine => &cfg.sources["machine"].clone(),
            };

            let mut ldap = src.connect()?;
            let (rs, _res) = ldap
                .search(&src.base,Scope::Subtree,&src.filter.unwrap(), src.attrs.unwrap())?
                .success()?;
            return Ok(rs);
        }
        return Err(anyhow!("System not initialized."));
    }
}

impl Session {
    pub fn new(cfg: &Config) -> Self {
        Session {
            cfg: Some(cfg.clone()),
        }
    }
}
