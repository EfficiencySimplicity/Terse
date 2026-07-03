use clap::{Parser, Subcommand};
use anyhow::Error;
use url::Url;

use std::path::PathBuf;

use crate::tui::App;
use crate::posts::Post;
use crate::network::{ServerList, SearchMenu, SearchResults};

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
    Server(ServerSubcommand)
}

#[derive(Subcommand)]
pub enum ServerSubcommand {
    Add {url: Url},
    Remove {url: Url},
    Set {idx: usize},
    List,
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
        match self {
            Cli::Search {query}    => Self::process_query(query),
            Cli::Stats             => Self::process_stats(),
            Cli::Pub {title, path} => Self::process_pub(title, path),
            Cli::Server(command)   => command.process(),
        }
        .inspect_err(|e| println!("{e}"))
        .ok();
    }

    fn process_query(words: Vec<String>) -> Result<(), Error> {

        // A note on the terminology; if these two are one line,
        // we get a 'temporary value dropped while borrowed' error;
        // https://stackoverflow.com/questions/71626083/error-e0716-temporary-value-dropped-while-borrowed
        // Which I believe is due to shaky knowledge of &mut and stuff
        // when making the selected() function in ServerList...

        let mut server_list = ServerList::from_config_file()?;
        let server = server_list.selected()?;
        let results = server.search(words)?;

        App::default().run(&mut SearchMenu::new(SearchResults::new(&server, results)))?;
        Ok(())
    }

    fn process_stats() -> Result<(), Error> {
        let mut server_list = ServerList::from_config_file()?;
        let server = server_list.selected()?;
        let stats = server.get_stats()?;
        
        println!("{stats}");
        Ok(())
    }

    // Maybe make custom errors for publishing, etc etc etc...
    fn process_pub(title: String, path: PathBuf) -> Result<(), Error> {

        // There could be a wrapper for fs errors that provides better printing
        // 'I couldn't read the path' is good enough, and after a semicolon; great!
        let content = std::fs::read_to_string(&path)
            .or(Err(Error::msg("I couldn't read the path you gave me")))?;

        // TODO: Is there a better way to manage content with the above match
        // to avoid unwrapping?
        let post = Post {title: title.clone(), content: content};

        let mut server_list = ServerList::from_config_file()?;
        let server = server_list.selected()?;
        // TODO: if Post gets a user field, we'll need to create the post in-server;
        // might not know what account you're in!
        // would mean an Option<user> for the best bet, I guess
        server.publish(post)?;

        println!("I was able to publish your post, \"{title}\"");
        Ok(())
    }
}

impl ServerSubcommand {
    fn process(self) -> Result<(), Error> {
        match self {
            Self::Add { url } => {
                ServerList::from_config_file()?.add_server(url.clone())?;
                println!("I successfully added {url} to the list of servers!");
            }
            Self::Remove { url } => {
                ServerList::from_config_file()?.remove_server(url.clone())?;
                println!("I successfully removed {} from the list of servers", url);
            }
            Self::Set { idx } => {
                ServerList::from_config_file()?.set_server(idx)?;
                // When this has additional info it'll be good to print it!
                println!("I successfully set the server to {idx}: ");
            }
            Self::List => {
                println!("{}", ServerList::from_config_file()?);
            }
        }
        Ok(())
    }
}