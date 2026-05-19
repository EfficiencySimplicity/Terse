use clap::{Parser, Subcommand};
use crate::stats::*;
use crate::posts::*;

// I manually detect which Cli to parse with and return it in this enum
#[derive(Debug)]
pub enum Cli {
    Commands(CommandsCli),
    Query(QueryCli),
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
    #[command(name = "--getpost")]
    GetPost {id: i32},
    #[command(name = "--pub")]
    Pub {title: String, path: String},
    // #[command(name = "--signup")]
    // Signup,
}

#[derive(Parser, Debug)]
pub struct QueryCli {
    #[clap(num_args = 1.., value_delimiter = ' ')]
    pub query: Vec<String>,
}

pub fn run_commands(command: CommandsCli) {
    match command.command {
        Commands::Stats => display_stats(),
        // TODO: This should not even be a command, maybe later
        Commands::GetPost{ id } => display_post(id),
        Commands::Pub{ title, path } => try_publish(title, path),
        _ => {}
    }
}

// https://github.com/clap-rs/clap/discussions/5725

// https://stackoverflow.com/questions/76315540/how-do-i-require-one-of-the-two-clap-options