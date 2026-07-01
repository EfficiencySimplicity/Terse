use clap::{Parser, Subcommand, Args};
use anyhow::Error;
use url::Url;

use std::path::PathBuf;

use crate::storage::ServerList;
use crate::tui::App;
use crate::queries::{SearchMenu, SearchResults};
use crate::posts::Post;

// https://docs.rs/clap/latest/clap/_derive/
// TODO: -s hand commands
// TODO: help

#[derive(Parser)]
pub enum Cli {
    #[command(name = "--search")]
    Query(Query),
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
            Cli::Query(query) => Self::process_query(query.words),
            Cli::Stats => Self::process_stats(),
            Cli::Pub {title, path} => Self::process_pub(title, path),
            Cli::Server(command) => command.process(),
        }
    }

    fn process_query(words: Vec<String>) {
        // A note on the terminology; if these two are one line,
        // we get a Temporary value dropped while borrowed error;
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
            Err(_) => {println!("Path not readable"); return}
            _ => {}
        }

        // TODO: Is there a better way to manage content with the above match
        // to avoid unwrapping?
        let post = Post {title, content: content.unwrap()};

        let mut server_list = ServerList::from_config_file().unwrap();
        let server = server_list.selected().unwrap();
        // TODO: if Post gets a user field, we'll need to create the post in-server;
        // might not know what account you're in!
        let results = server.publish(post);

        match results {
            Ok(_) => println!("Post published successfully!"),
            Err(e) => println!("Error in publishing post: {e}")
        }
    }
}

impl ServerSubcommand {
    fn process(self) {
        match self {
            Self::Add { url } => {
                match || -> Result<(), Error> {Ok(ServerList::from_config_file()?.add_server(url.clone())?)}() {
                    Ok(_) => println!("Successfully added server {url}!"),
                    Err(e) => println!("{e}")
                }
            }
            Self::List => {
                match || -> Result<(), Error> {Ok(println!("{}", ServerList::from_config_file()?))}() {
                    Err(e) => println!("Error in listing servers: {e}"),
                    _ => {},
                }
            }
            Self::Remove { url } => {
                match || -> Result<(), Error> {Ok(ServerList::from_config_file()?.remove_server(url.clone())?)}() {
                    Ok(_) => println!("I successfully removed the server {}", url),
                    Err(e) => println!("{e}"),
                }
            }
        }
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