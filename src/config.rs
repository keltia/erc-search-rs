//! Main configuration management and loading
//!
use std::collections::HashMap;
use std::fs;

use anyhow::Result;
use serde::Deserialize;

use crate::source::Source;

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
    pub fn new() -> Config {
        Config {
            verbose: Some(false),
            sources: HashMap::new(),
        }
    }

    // That way, we do not panic whatever happens and return an empty Config.
    pub fn load(fname: &str) -> anyhow::Result<Config> {
        let content = fs::read_to_string(fname)?;

        println!("{:?}", content);

        let s: Config = toml::from_str(&content).unwrap_or_default();
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
