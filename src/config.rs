use std::fs;

use serde::Deserialize;
use toml::value::Table;

#[derive(Debug, Deserialize)]
pub struct Source {
    domain: String,
    site: String,
    port: Option<u16>,
    base: String,
    filter: Option<String>,
    attrs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub verbose: Option<bool>,
    pub sources: Option<Table>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            verbose: Option::None,
            sources: Option::None,
        }
    }

    pub fn biip(self: &Config) {
        println!("Biiip");
    }
}

// That way, we do not panic whatever happens and return an empty Config.
pub fn load_config(fname: &str) -> Config {
    let nul = Config::new();
    let content = fs::read_to_string(fname);

    println!("{:?}", content);

    let content = match content {
        Ok(content) => content,
        Err(_) => return nul,
    };
    let cfg = toml::from_str(&content);
    match cfg {
        Ok(cfg) => cfg,
        Err(_) => return nul,
    }
}

