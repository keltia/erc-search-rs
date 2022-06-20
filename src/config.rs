//! Main configuration management and loading
//!
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context,Result};
use clap::crate_name;
use home::home_dir;

/// Default configuration filename
const CONFIG: &str = "config.toml";

#[cfg(unix)]
const BASEDIR: &str = ".config";

/// Main struct holding configurations
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub verbose: Option<bool>,
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
            verbose: Some(false),
            sources: HashMap::new(),
        }
    }

    /// Load the specified config file
    pub fn load(fname: &PathBuf) -> anyhow::Result<Config> {
        let content = fs::read_to_string(fname)?;

        println!("{:?}", content);

        let s: Config = toml::from_str(&content).unwrap_or_default();
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
    pub fn default_file() -> anyhow::Result<PathBuf> {
        let homedir = home_dir().unwrap();

        let def: PathBuf = [
            homedir,
            PathBuf::from(crate_name!()),
            PathBuf::from(CONFIG),
        ]   .iter()
            .collect();
        Ok(def)
    }
*/
    pub fn load(fname: &str) -> anyhow::Result<Config<'cnf>> {
        let nul = Config::new();
        let content = fs::read_to_string(fname)?;
        println!("{:?}", content);
        let c = toml::from_str(&content)?;
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new(){
        let a = Config::new();
        assert_eq!(a, Config {
            verbose: Some(false),
            sources: HashMap::new(),
        });
        println!("{:?}", a)
    }

    #[test]
    fn test_new() {
        let a = Config::new();
        assert_eq!(
            Config {
                verbose: Some(false),
                sources: HashMap::new(),
            },
            a
        )
    }
}
