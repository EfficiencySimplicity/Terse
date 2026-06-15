pub mod cli;
use cli::*;

pub mod stats;
pub mod posts;
pub mod tui;

pub mod queries;
use queries::api::*;

fn main() {
    let cli = get_cli();

    match cli {
        Cli::Query(cli) => process_query(cli.query),
        Cli::Commands(cli) => process_command(cli),
    }
}
