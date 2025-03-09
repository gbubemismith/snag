use std::process;

use clap::Parser;
use snag::Cli;

fn main() {
    let args = Cli::parse();
    if let Err(err) = args.run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
