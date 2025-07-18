// src/main.rs
/*
 * Main executable for ArtificialMasterpieceGeneratorEngineNext
 */

use clap::Parser;
use artificialmasterpiecegeneratorenginenext::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialMasterpieceGeneratorEngineNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
