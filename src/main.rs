pub mod cli;
use cli::Cli;

pub mod tui;
pub mod network;
pub mod posts;

fn main() {
    let cli = Cli::from_args();
    cli.process();
}
