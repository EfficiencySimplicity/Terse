pub mod cli;
use cli::*;

pub mod posts;
pub mod tui;
pub mod network;

pub mod queries;
pub mod storage;


fn main() {
    let cli = Cli::from_args();
    cli.process();
}
