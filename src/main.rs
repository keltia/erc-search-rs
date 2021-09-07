use std::fs;

//use toml::Value;
use clap::{AppSettings, Clap};
use serde::Deserialize;
use toml::value::Table;

//use serde_derive::Deserialize;

/// Help message
#[derive(Debug, Clap)]
#[clap(name = "erc-search", about = "Search internal LDAP/AD.")]
#[clap(version = "0.1.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opt {
    /// configuration file
    #[clap(short = 'c', long)]
    config: String,
    /// debug mode
    #[clap(short = 'D', long = "debug")]
    debug: bool,
    /// Include mail search
    #[clap(short = 'M', long = "incl-mail")]
    incl_mail: bool,
    /// Verbose mode
    #[clap(short = 'v', long)]
    verbose: bool,
    /// Display version and exit
    #[clap(short = 'V', long = "version")]
    version: bool,
    /// Search for workstation
    #[clap(short, long = "workstation")]
    workstation: bool,
    /// string to search for
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

impl Config {
    fn new() -> Config {
        Config {
            verbose: Option::None,
            sources: Option::None,
        }
    }

    fn new_from(a: Config) -> Config {
        Config {
            verbose: a.verbose,
            sources: a.sources,
        }
    }
}

#[derive(Debug)]
struct Context {
    src: String,
    cnx: Option<ldap3::LdapConn>,
}

impl Context {
    fn new() -> Context {
        Context {
            src: "".to_string(),
            cnx: Option::None,
        }
    }
}

fn verbose(s: &str) {
    println!("{}", s);
}

// That way, we do not panic whatever happens and return an empty Config.
fn load_config(fname: &str) -> Config {
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

fn main() {
    let ctx = Context::new();
    let opts: Opt = Opt::parse();
    let mut cfg = load_config("src/config.toml");

    verbose("Hello world");
    println!("{:?}", ctx);
    println!("{:?}", opts);
    println!("{:?}", cfg.sources);

    cfg.verbose = Some(opts.verbose);

    //cfg.list();

    verbose("Mode verbeux engagé");
}
