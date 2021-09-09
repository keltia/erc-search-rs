mod config;

use crate::config::*;

//use toml::Value;
use clap::{AppSettings, Clap};

//use serde_derive::Deserialize;

/// Help message
#[derive(Debug, Clap)]
#[clap(name = "erc-search", about = "Search internal LDAP/AD.")]
#[clap(version = "0.1.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opt {
    /// configuration file
    #[clap(short = 'c', long)]
    config: Option<String>,
    /// debug mode
    #[clap(short = 'D', long = "debug")]
    debug: Option<bool>,
    /// Include mail search
    #[clap(short = 'M', long = "incl-mail")]
    incl_mail: Option<bool>,
    /// Verbose mode
    #[clap(short = 'v', long)]
    verbose: bool,
    /// Display version and exit
    #[clap(short = 'V', long = "version")]
    version: Option<bool>,
    /// Search for workstation
    #[clap(short, long = "workstation")]
    workstation: Option<bool>,
    /// string to search for
    what: String,
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

fn main() {
    let ctx = Context::new();
    let opts: Opt = Opt::parse();
    let mut cfg = load_config("src/config.toml");

    verbose("Hello world");
    println!("{:?}", ctx);
    println!("{:?}", opts);
    println!("{:?}", cfg.sources);

    cfg.verbose = Some(opts.verbose);

    cfg.biip();

    verbose("Mode verbeux engagé");
}
