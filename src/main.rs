//use toml::Value;
use clap::{AppSettings, Clap};

use crate::config::*;

mod config;

#[macro_use]
mod macros;

/// Help message
#[derive(Debug, Clap)]
#[clap(name = "erc-search", about = "Search internal LDAP/AD.")]
#[clap(version = "0.1.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
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
    pub v: bool,
    pub src: String,
    pub cnx: Option<ldap3::LdapConn>,
}

impl Context {
    fn new() -> Context {
        Context {
            v: false,
            src: "".to_string(),
            cnx: Option::None,
        }
    }
}

fn main() {
    let mut ctx = Context::new();

    let opts = Opts::parse();
    let cfg = Config::load("src/config.toml");

    ctx.v = opts.verbose;

    verbose!(ctx, "Hello world");
    println!("{:?}", ctx);
    println!("{:?}", opts);
    println!("{:?}", cfg.sources);
    verbose!(ctx, "Mode verbeux engagé");
}
