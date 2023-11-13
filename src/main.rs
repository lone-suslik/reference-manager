mod fs;
mod hash;
use clap::{Parser, Subcommand};
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

fn parse_path_or_exit(cli: Cli) -> io::Result<PathBuf> {
    match std::fs::canonicalize(cli.path) {
        Err(e) => {
            eprintln!(
                "Error: unable to canonicalize the path. The error is: \n{:#?}",
                e
            );
            std::process::exit(1);
        }
        p => return p,
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let full_path = parse_path_or_exit(cli)?;
    let res = fs::collect_reference_objects(&full_path)?;
    println!("{:#?}", res);
    return Ok(());
}
