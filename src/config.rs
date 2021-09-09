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

