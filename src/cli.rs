use clap::{Parser, Subcommand};
use anyhow::Error;
use url::Url;

use std::path::PathBuf;

use crate::network::ServerList;
use crate::tui::App;
use crate::queries::{SearchMenu, SearchResults};
use crate::posts::Post;

// https://docs.rs/clap/latest/clap/_derive/
// TODO: -s hand commands
// TODO: help
// NOTE: I could move stats into the server and so go --server stats?...

// This helped me get a Vec<String> into an enum:
// https://github.com/clap-rs/clap/blob/master/examples/tutorial_derive/03_04_subcommands.rs

#[derive(Parser)]
pub enum Cli {
    #[command(name = "--search")]
    Query {query: Vec<String>},
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
            Cli::Query {query}     => Self::process_query(query),
            Cli::Stats             => Self::process_stats(),
            Cli::Pub {title, path} => Self::process_pub(title, path),
            Cli::Server(command)   => command.process(),
        }
    }

    fn process_query(words: Vec<String>) {

        // A note on the terminology; if these two are one line,
        // we get a 'temporary value dropped while borrowed' error;
        // https://stackoverflow.com/questions/71626083/error-e0716-temporary-value-dropped-while-borrowed
        // Which I believe is due to shaky knowledge of &mut and stuff
        // when making the selected() function in ServerList...

        let mut server_list = ServerList::from_config_file().unwrap();
        let server = server_list.selected().unwrap();
        let results = server.search(words);

        match results {
            Ok(r) => {App::default().run(&mut SearchMenu::new(SearchResults::new(&server, r))).unwrap()}
            Err(e) => {println!("Error: {e}")}
        }
    }

    fn process_stats() {
        let mut server_list = ServerList::from_config_file().unwrap();
        let server = server_list.selected().unwrap();
        let results = server.get_stats();

        match results {
            Ok(stats) => {println!("{stats}")}
            Err(e) => {println!("Error in getting stats: {e}")}
        }
    }

    fn process_pub(title: String, path: PathBuf) {

        let content = std::fs::read_to_string(&path);
        
        match content {
            Err(_) => {println!("I wasn't able to read anything from that path"); return}
            _ => {}
        }

        // TODO: Is there a better way to manage content with the above match
        // to avoid unwrapping?
        let post = Post {title: title.clone(), content: content.unwrap()};

        let mut server_list = ServerList::from_config_file().unwrap();
        let server = server_list.selected().unwrap();
        // TODO: if Post gets a user field, we'll need to create the post in-server;
        // might not know what account you're in!
        // would mean an Option<user> for the best bet, I guess
        let results = server.publish(post);

        match results {
            Ok(_) => println!("I was able to publish your post, \"{title}\""),
            Err(e) => println!("I got an error when trying to publish your post: {e}")
        }
    }
}

impl ServerSubcommand {
    fn process(self) {
        match self {
            Self::Add { url } => {
                match || -> Result<(), Error> {Ok(ServerList::from_config_file()?.add_server(url.clone())?)}() {
                    Ok(_) => println!("I successfully added {url} to the list of servers!"),
                    Err(e) => println!("{e}")
                }
            }
            Self::List => {
                match || -> Result<(), Error> {Ok(println!("{}", ServerList::from_config_file()?))}() {
                    Err(e) => println!("I got an error when trying to list servers: {e}"),
                    _ => {},
                }
            }
            Self::Remove { url } => {
                match || -> Result<(), Error> {Ok(ServerList::from_config_file()?.remove_server(url.clone())?)}() {
                    Ok(_) => println!("I successfully removed the {} from the list of servers", url),
                    Err(e) => println!("{e}"),
                }
            }
        }
    }
}