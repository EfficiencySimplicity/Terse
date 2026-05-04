use clap::{Parser, Subcommand};

#[derive(Debug)]
pub enum Cli {
    Commands(CommandsCli),
    Query(QueryCli),
}

#[derive(Parser, Debug)]
pub struct CommandsCli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    #[command(name = "--stats")]
    Stats,
    #[command(name = "--pub")]
    Pub,
    #[command(name = "--signup")]
    Signup,
}

#[derive(Parser, Debug)]
pub struct QueryCli {
    #[clap(num_args = 1.., value_delimiter = ' ')]
    pub query: Vec<String>,
}

// https://github.com/clap-rs/clap/discussions/5725

// https://stackoverflow.com/questions/76315540/how-do-i-require-one-of-the-two-clap-options