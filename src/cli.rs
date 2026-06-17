use clap::{Parser, Subcommand, Args};
use crate::stats::*;
use crate::posts::*;

// https://docs.rs/clap/latest/clap/_derive/

#[derive(Parser)]
pub enum Cli {
    #[command(name = "--search")]
    Query(Query),
    #[command(name = "--stats")]
    Stats,
    #[command(name = "--pub")]
    Pub {title: String, path: String},
}

impl Cli {
    pub fn from_args() -> Self {
        if let Some(command) = std::env::args().nth(1) {
            // Why does pythonexamples.org have rust tutorials?!
            // https://pythonexamples.org/rust/how-to-get-first-n-characters-in-string
            // https://www.dotnetperls.com/starts-with-rust
            if !command.starts_with("-") {
                // search shortcut! Stick a --search in there and parse it!
                let mut search_insert = std::env::args().into_iter().collect::<Vec<String>>();
                search_insert.insert(1, "--search".to_string());
                return Self::parse_from(search_insert);
            }
        }

        return Self::parse()
    }
}

#[derive(Args)]
pub struct Query {
    pub words: Vec<String>,
}
// #[derive(Parser, Debug)]
// pub struct QueryCli {
//     #[clap(num_args = 1.., value_delimiter = ' ')]
//     pub query: Vec<String>,
// }

// https://github.com/clap-rs/clap/discussions/5725

// https://stackoverflow.com/questions/76315540/how-do-i-require-one-of-the-two-clap-options