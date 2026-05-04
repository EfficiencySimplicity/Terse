use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "--stats")]
    Stats,
    #[command(name = "--pub")]
    Pub,
    #[command(name = "--signup")]
    Signup,
}