use std::fs;

//use toml::Value;
use serde::Deserialize;
use structopt::StructOpt;
use toml::value::Table;

//use serde_derive::Deserialize;

#[derive(Debug, StructOpt)]
#[structopt(name = "erc-search", about = "Search internal LDAP/AD.")]
struct Opt {
    #[structopt(short = "D", help = "debug mode")]
    debug: bool,
    #[structopt(short = "M", help = "Include mail search")]
    incl_mail: bool,
    #[structopt(short = "v", help = "Verbose mode")]
    verbose: bool,
    #[structopt(short = "V", long, help = "Display version and exit")]
    version: bool,
    #[structopt(short, help = "Search for workstation")]
    workstation: bool,
    #[structopt(help = "string to search for")]
    what: String,
}

#[derive(Debug, Deserialize)]
struct Source {
    domain: String,
    site: String,
    port: Option<u16>,
    base: String,
    filter: Option<String>,
    attrs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Config {
    verbose: Option<bool>,
    sources: Option<Table>,
}

fn verbose(s: &str) {
    println!("{}", s);
}

fn load_config(fname: &str) -> Config {
    let nul = Config {
        verbose: Option::None,
        sources: Option::None,
    };

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

fn main() {
    let opt = Opt::from_args();
    let cfg = load_config("src/config.toml");

    verbose("Hello world");
    println!("{:?}", opt);
    println!("{:?}", cfg.sources);

    verbose("Mode verbeux engagé");
}
