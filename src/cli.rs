use clap::{Parser, Subcommand};
use crate::stats::*;
use crate::posts::*;

// I manually detect which Cli to parse with and return it in this enum
#[derive(Debug)]
pub enum Cli {
    Commands(CommandsCli),
    Query(QueryCli),
}

impl Cli {
    pub fn from_args() -> Self {
        if let Some(command) = std::env::args().nth(1) {
            // Why does pythonexamples.org have rust tutorials?!
            // https://pythonexamples.org/rust/how-to-get-first-n-characters-in-string
            // https://www.dotnetperls.com/starts-with-rust
            if command.starts_with("-") {
                return Cli::Commands(CommandsCli::parse())
            }
        }

        return Cli::Query(QueryCli::parse())
    }
}

// TODO: if this doesn't change, I could just use the Commands enum
#[derive(Parser, Debug)]
pub struct CommandsCli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    #[command(name = "--stats")]
    Stats,
    #[command(name = "--pub")]
    Pub {title: String, path: String},
}

impl CommandsCli {
    pub fn process(self) {
        match self.command {
            Commands::Stats => display_stats(),
            Commands::Pub{ title, path } => try_publish(title, path),
        }
    }
}

#[derive(Parser, Debug)]
pub struct QueryCli {
    #[clap(num_args = 1.., value_delimiter = ' ')]
    pub query: Vec<String>,
}

// https://github.com/clap-rs/clap/discussions/5725

// https://stackoverflow.com/questions/76315540/how-do-i-require-one-of-the-two-clap-options