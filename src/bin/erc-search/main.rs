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
    workstation: bool,
    /// string to search for
    what: String,
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
    let mut cfg = match cfg {
        Ok(c) => c,
        Err(e) => panic!("Need a config file! {}", e),
    };
    if opts.workstation {
        cfg.sfor = Search::Machine;
    }
    cfg
}


fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    // Load default config if nothing is specified
    let cfg = get_config(&opts);

    println!("{:?}", opts);
    println!("{:?}", cfg);
    verbose!(cfg, "Verbose mode.");
    println!("{:?}", cfg.sources);

    // Search data
    let what = opts.what;

    let res = Session::new(&cfg)?.search(what)?;

    Ok(display_results(res))
}
