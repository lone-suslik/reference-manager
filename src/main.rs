#![allow(unused_variables, dead_code)]

mod asset;
mod config;
mod hash;
mod iterator;
use clap::{Parser, Subcommand};
use config::Config;
use iterator::collect_reference_objects;
use std::io;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// provide a path to the reference directory
    #[arg(value_name = "REF_DIR")]
    path: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn parse_path_or_exit(path: String) -> io::Result<PathBuf> {
    match std::fs::canonicalize(path) {
        Err(e) => {
            eprintln!(
                "Error: unable to canonicalize the path. Most likely base directory is missing. Error: \n{:#?}",
                e
            );
            std::process::exit(1);
        }
        p => return p,
    }
}

fn main() -> io::Result<()> {
    // parse cli
    let cli = Cli::parse();
    let config: Config;

    // load config
    match config::load_config() {
        Ok(c) => config = c,
        Err(e) => {
            eprintln!("Failed to load configuration file. Error: {:#?}", e);
            std::process::exit(1);
        }
    }

    let full_path = parse_path_or_exit(config.base_path)?;
    let ref_objects = collect_reference_objects(&full_path)?;

    for a in ref_objects {
        let asset = a?;
        eprintln!("{:#?}", asset);
    }

    return Ok(());
}
