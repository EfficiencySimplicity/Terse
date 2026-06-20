use clap::{Parser, Args};

use crate::network::Server;
use crate::tui::App;
use crate::queries::{SearchMenu, SearchResults};
use crate::posts::Post;
// TODO: we need to import Url with Server all the time... shouldn't there be an additional string method?
// Well, we eventually won't be creating servers willy-nilly on-demand.
use url::Url;

// https://docs.rs/clap/latest/clap/_derive/

#[derive(Parser)]
pub enum Cli {
    #[command(name = "--search")]
    Query(Query),
    #[command(name = "--stats")]
    Stats,
    #[command(name = "--pub")]
    // TODO: can this path be parsed as an actual Path type?
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

    pub fn process(self) {
        match self {
            Cli::Query(query) => Self::process_query(query.words),
            Cli::Stats => Self::process_stats(),
            Cli::Pub {title, path} => Self::process_pub(title, path)
        }
    }

    fn process_query(words: Vec<String>) {
        let server = Server::new(Url::parse("http://localhost:3000").unwrap());
        let results = server.search(words);

        match results {
            Ok(r) => {App::default().run(&mut SearchMenu::new(SearchResults::new(r))).unwrap()}
            Err(e) => {println!("Error: {e}")}
        }
    }

    fn process_stats() {
        let server = Server::new(Url::parse("http://localhost:3000").unwrap());
        let results = server.get_stats();

        match results {
            Ok(stats) => {println!("{stats}")}
            Err(e) => {println!("Error in getting stats: {e}")}
        }
    }

    fn process_pub(title: String, path: String) {

        let content = std::fs::read_to_string(&path);
        
        match content {
            Err(_) => {println!("Path not readable"); return}
            _ => {}
        }

        // TODO: Is there a better way to manage content with the above match
        // to avoid unwrapping?
        let post = Post {title, content: content.unwrap()};

        let server = Server::new(Url::parse("http://localhost:3000").unwrap());
        // TODO: if Post gets a user field, we'll need to create the post in-server;
        // might not know what account you're in!
        let results = server.publish(post);

        match results {
            Ok(_) => println!("Post published successfully!"),
            Err(e) => println!("Error in publishing post: {e}")
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