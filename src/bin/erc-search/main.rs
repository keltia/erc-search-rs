use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{crate_authors, crate_version, Parser};
use ldap3::{LdapConn, LdapError, LdapResult};

use erc_search::config::Config;
use erc_search::display::display_results;
use erc_search::display_results;
use erc_search::session::{Search, Session};

#[macro_use]
mod macros;

/// Help message
#[derive(Parser, Debug)]
#[clap(name = "erc-search", about = "Search internal LDAP/AD.")]
#[clap(version = crate_version ! (), author = crate_authors ! ())]
struct Opts {
    /// configuration file
    #[clap(short = 'c', long)]
    config: Option<PathBuf>,
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

#[derive(Debug, Default)]
struct Ctx {
    pub v: bool,
    pub src: String,
    pub cfg: Config,
}

impl Ctx {
    pub fn new() -> Ctx {
        Ctx {
            cfg: Config::new(),
            ..Default::default()
        }
    }
}

fn get_config(opts: &Opts) -> Config {
    // Load default config if nothing is specified
    let cfg = match &opts.config {
        Some(c) => {
            Config::load(&c).with_context(|| format!("No file {:?}", c))
        }
        None => {
            let cnf = Config::default_file().unwrap();
            Config::load(&cnf).with_context(|| format!("No file {:?}", cnf))
        }
    };

    // We must have a valid configuration, an error means no default one
    let cfg = match cfg {
        Ok(c) => c,
        Err(e) => panic!("Need a config file! {}", e),
    };
    cfg
}

fn main() {
    let mut ctx = Ctx::new();

    verbose!(ctx, "Hello world");

    let opts: Opts = Opts::parse();

    // Load default config if nothing is specified
    let cfg = get_config(&opts);

    println!("{:?}", cfg.sources);

    ctx.v = opts.verbose;

    verbose!(ctx, "Hello world");
    println!("{:?}", ctx);
    println!("{:?}", opts);
    println!("{:?}", cfg);
    verbose!(ctx, "Mode verbeux engagé");

    // Default search type
    let s = match opts.workstation {
        Some(true) => Search::Machine(&opts.what),
        _ => Search::People(&opts.what),
    };

    let _res = s.doit(&ctx, &opts.what);
}
