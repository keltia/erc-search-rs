//! Main configuration management and loading
//!
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::crate_name;
use home::home_dir;
use serde::Deserialize;

use crate::session::Search;
use crate::source::Source;

/// Default configuration filename
const CONFIG: &str = "config.toml";

#[cfg(unix)]
const BASEDIR: &str = ".config";

/// Main struct holding configurations
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Config {
    /// Searching for what type of data?
    pub sfor: Search,
    /// Do we want verbose by default?
    pub verbose: bool,
    /// Different sources to search into
    pub sources: HashMap<String, Source>,
}

/// `Default` is for `unwrap_or_default()`.
impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Methods for `Config`
impl Config {
    /// Returns an empty struct
    pub fn new() -> Config {
        Config {
            sfor: Search::Person,
            verbose: false,
            sources: HashMap::new(),
        }
    }

    /// Load the specified config file
    pub fn load(fname: &PathBuf) -> Result<Config> {
        let content = fs::read_to_string(fname)?;

        let s: Config = toml::from_str(&content)?;
        Ok(s)
    }

    /// Returns the path of the default config file
    #[cfg(unix)]
    pub fn default_file() -> Result<PathBuf> {
        let homedir = home_dir().unwrap();
        let def: PathBuf = [
            homedir,
            PathBuf::from(BASEDIR),
            PathBuf::from(crate_name!()),
            PathBuf::from(CONFIG),
        ]   .iter()
            .collect();
        Ok(def)
    }

    /// Returns the path of the default config file
    #[cfg(windows)]
    pub fn default_file() -> Result<PathBuf> {
        let homedir = home_dir().unwrap();

        let def: PathBuf = [
            homedir,
            PathBuf::from(crate_name!()),
            PathBuf::from(CONFIG),
        ].iter()
            .collect();
        Ok(def)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new(){
        let a = Config::new();
        assert_eq!(a, Config {
            sfor: Search::Person,
            verbose: false,
            sources: HashMap::new(),
        });
        println!("{:?}", a)
    }

    #[test]
    fn test_config_load() {
        let cn = PathBuf::from("config.toml");
        let cfg = Config::load(&cn);
        assert!(cfg.is_ok());

        let cfg = cfg.unwrap();
        assert!(!cfg.verbose);
        assert!(!cfg.sources.is_empty());
    }
}
