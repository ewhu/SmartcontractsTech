// src/main.rs
/*
 * Main executable for SmartcontractsTech
 */

use clap::Parser;
use smartcontractstech::{Result, run};

#[derive(Parser)]
#[command(version, about = SmartcontractsTech - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
