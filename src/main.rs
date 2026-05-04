pub mod cli;
use cli::*;

use clap::Parser;

fn main() {
    let cli = Cli::parse();
}
