pub mod cli;
use crate::cli::*;

pub mod queries;
use queries::*;

pub mod commands;
use commands::*;

use std::env::Args;
use clap::Parser;

fn get_cli(args: Args) -> Cli {
    if let Some(command) = std::env::args().nth(1) {
        // https://pythonexamples.org/rust/how-to-get-first-n-characters-in-string
        // https://www.dotnetperls.com/starts-with-rust
        if command.starts_with("-") {
            return Cli::Commands(CommandsCli::parse())
        }
    }

    return Cli::Query(QueryCli::parse())
}

fn main() {
    let cli = get_cli(std::env::args());
    println!("{:?}", cli);

    match cli {
        Cli::Query(cli) => run_query_app(cli.query),
        Cli::Commands(cli) => run_commands(cli),
    }
}
