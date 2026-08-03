use clap::{Parser, Subcommand};
use anyhow::Error;
use url::Url;

use std::path::PathBuf;

use crate::tui::App;
use crate::posts::Post;
use crate::network::{Account, ServerList, SearchMenu, SearchResults};

pub mod server_subcommand;
use server_subcommand::*;

#[cfg(debug_assertions)]
pub mod dev_subcommand;
#[cfg(debug_assertions)]
use dev_subcommand::*;

// https://docs.rs/clap/latest/clap/_derive/
// TODO: -s hand commands
// TODO: help
// NOTE: I could move stats into the server and so go --server stats?...

// This helped me get a Vec<String> into an enum:
// https://github.com/clap-rs/clap/blob/master/examples/tutorial_derive/03_04_subcommands.rs

#[derive(Parser)]
pub enum Cli {
    #[command(name = "--search")]
    Search {query: Vec<String>},
    #[command(name = "--stats")]
    Stats,
    #[command(name = "--pub")]
    Pub {title: String, path: PathBuf},
    #[command(subcommand, name = "--server")]
    Server(ServerSubcommand),
    #[command(name = "--whoami")]
    Whoami,
    
    #[cfg(debug_assertions)]
    // eeyou do not GET access to this pro-prietary subcommando!!!
    #[command(subcommand, name = "--dev", about = "Special developer commands to do things faster")]
    Dev(DevSubcommand),
}

impl Cli {
    pub fn from_args() -> Self {

        // if the first argument isn't a flag (doesn't start with -)
        // that's a shortcut for searching; so we insert a --search command
        // and parse from it, for your convenience!

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

    pub fn process(self) {
        // Having this here does mean that some commands that don't need
        // disk access at all could fail, but it is WORTH IT.
        let mut server_list = match ServerList::from_config_file() {
            Ok(s) => s,
            Err(_) => {
                eprintln!("There was an error reading from disk :(");
                return
            }
        };

        let run_result = match self {
            Cli::Search {query}    => Self::process_query(&server_list, query),
            Cli::Stats             => Self::process_stats(&server_list),
            Cli::Pub {title, path} => Self::process_pub(&server_list, title, path),
            Cli::Server(command)   => command.process(&mut server_list),
            //Cli::Account(command)  => command.process(&mut server_list),
            Cli::Whoami            => Self::process_whoami(&server_list),

            #[cfg(debug_assertions)]
            Cli::Dev(command)      => command.process(&mut server_list),
        };

        match run_result {
            Ok(_) => if let Err(e) = server_list.store() {
                println!("Error writing to disk: {e}")
            }
            Err(e) => eprintln!("{e}")
        }
    }

    fn process_query(server_list: &ServerList, words: Vec<String>) -> Result<(), Error> {

        let server = server_list.clone_selected()?;
        let results = server.search(words)?;

        App::default().run(&mut SearchMenu::new(SearchResults::new(&server, results)))?;
        Ok(())
    }

    fn process_stats(server_list: &ServerList) -> Result<(), Error> {
        let server = server_list.clone_selected()?;
        let stats = server.get_stats()?;
        
        println!("{stats}");
        Ok(())
    }

    // Maybe make custom errors for publishing, etc etc etc...
    fn process_pub(server_list: &ServerList, title: String, path: PathBuf) -> Result<(), Error> {

        // There could be a wrapper for fs errors that provides better printing
        // 'I couldn't read the path' is good enough, and after a semicolon; great!
        let content = std::fs::read_to_string(&path)
            .or(Err(Error::msg("I couldn't read the path you gave me")))?;

        // TODO: Is there a better way to manage content with the above match
        // to avoid unwrapping?
        let post = Post {title: title.clone(), content: content};

        let server = server_list.clone_selected()?;
        // TODO: if Post gets a user field, we'll need to create the post in-server;
        // might not know what account you're in!
        // would mean an Option<user> for the best bet, I guess
        server.publish(post)?;

        println!("I was able to publish your post, \"{title}\"");
        Ok(())
    }

    fn process_whoami(server_list: &ServerList) -> Result<(), Error> {
        let maybe_server = server_list.clone_selected();
        match maybe_server {
            Ok(server) => {
                println!("You are on {}", server.identifier_string());
                println!("TODO: have the server store the current account");
            },
            Err(_) => {
                // You DEFINITELY have none; it can't be outta bounds, right?!
                eprintln!("I couldn't get the current server... You might have none!");
            }
        }
        Ok(())
    }
}