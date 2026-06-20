pub mod cli;
use cli::*;

pub mod posts;
pub mod tui;
pub mod network;

pub mod queries;
use queries::*;

fn main() {
    let cli = Cli::from_args();

    // TODO: this code, in the CLI; cli::process()
    // or leave it here?. . . I mean, yea, each processor is in
    // a different module, and this logic must be somewhere,
    // Why not in main.rs???
    match cli {
        // Because empty functions need empty files to accommodate them!
        // stick it in CLI!
        Cli::Query(query) => process_query(query.words),
        _ => println!("AAAA")
        //Cli::Commands(commands_cli) => commands_cli.process(),
    }
}
