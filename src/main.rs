pub mod cli;
use cli::*;

pub mod posts;
pub mod tui;
pub mod network;

pub mod queries;

pub mod utils;

fn main() {
    let cli = Cli::from_args();
    cli.process();
}
