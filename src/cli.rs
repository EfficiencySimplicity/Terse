use clap::{Parser, Subcommand};
use anyhow::Error;
use url::Url;

use text_io::read;

use std::path::PathBuf;

use crate::tui::App;
use crate::posts::Post;
use crate::network::{Account, ServerList, SearchMenu, SearchResults};

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
    #[command(subcommand, name = "--account")]
    Account(AccountSubcommand),
}

#[derive(Subcommand)]
pub enum ServerSubcommand {
    Add {url: Url},
    Remove {url: Url},
    Set {idx: usize},
    List,
}

#[derive(Subcommand)]
pub enum AccountSubcommand {
    New,
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
            Cli::Account(command)  => command.process(),
        }
        .inspect_err(|e| println!("{e}"))
        .ok();
    }

    fn process_query(words: Vec<String>) -> Result<(), Error> {

        let server = ServerList::from_config_file()?.extract_selected()?;
        let results = server.search(words)?;

        App::default().run(&mut SearchMenu::new(SearchResults::new(&server, results)))?;
        Ok(())
    }

    fn process_stats() -> Result<(), Error> {
        let server = ServerList::from_config_file()?.extract_selected()?;
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

        let server = ServerList::from_config_file()?.extract_selected()?;
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
                let mut server_list = ServerList::from_config_file()?;
                let server = server_list.set_server(idx)?;
                // If this wanted more stats... I'd just not globally error;
                // I'd say "I successfully set the server to 4: [couldn't get name]""
                println!("I successfully set the server to {idx}: {}", server.url());
            }
            Self::List => {
                println!("{}", ServerList::from_config_file()?);
            }
        }
        Ok(())
    }
}

impl AccountSubcommand {
    fn process(self) -> Result<(), Error> {
        match self {
            Self::New => {
                let mut server_list = ServerList::from_config_file()?;
                let server = server_list.selected()?;

                println!("Creating new account on server {}:\n", server.url());

                println!("Email: ");
                let email: String = read!("{}\n");
                println!("");

                let mut username: String;

                loop {
                    println!("Username: ");
                    username = read!("{}\n");

                    match server.account_exists(&username) {
                        Ok(exists) => {
                            if exists {
                                println!("An account by the name of {username} already exists on {}", server.url())
                            } else {
                                println!("");
                                break
                            }
                        }
                        Err(e) => {
                            println!("I got an error when trying to check if an account with that username exists on {}:", server.url());
                            println!("{e}")
                        }
                    }
                    println!("");
                }

                println!("Password: ");
                let password: String = read!("{}\n");
                println!("");

                // TODO: have a few types of errors that the server can json back;
                // or it it sorta unknown what the server'll do?
                // Like, it could auto-sign ya up, or tell you to go to a link...
                let return_message = server.create_account(Account::new(email, username, password))?;

                println!("I asked the server to create your account, and it said:");
                println!("{}", return_message);

                // server request new account...
            }
        }
        Ok(())
    }
}