pub mod cli;
use cli::*;

pub mod stats;
pub mod posts;
pub mod tui;

pub mod queries;
use queries::api::*;

fn main() {
    let cli = Cli::from_args();

    // TODO: this code, in the CLI; cli::process()
    // or leave it here?. . . I mean, yea, each processor is in
    // a different module, and this logic must be somewhere,
    // Why not in main.rs???
    match cli {
        Cli::Query(query_cli) => process_query(query_cli.query),
        Cli::Commands(commands_cli) => commands_cli.process(),
    }
}
