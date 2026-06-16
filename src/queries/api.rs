use serde::{ Deserialize };
use reqwest::{Error};
use ratatui::prelude::{Line, Span, Text};
// Glad to remove this! TODO
use crate::queries::tui::*;

// in the SQL, the author'll be an Id, but it JOINS into an object
// TODO: age (in case the link gets invalidated between searching and entering)
#[derive(Deserialize, Debug)]
pub struct SearchResult {
    pub title: String,
    pub postid: u16,
}

// https://www.reddit.com/r/rust/comments/7zm0j2/intofrom_for_nonconsuming_conversions/
impl<'a> From<&'a SearchResult> for Text<'a> {
    fn from(value: &'a SearchResult) -> Self {
        let text = Text::from(vec![
            Line::from(" "),
            Line::from(Span::from(&value.title)),
            //Line::from(value.author.clone()).red(),
            Line::from(" "),
        ]);

        return text;
    }
}

// NOTE: does this need ta be a Vec<String>? 'cause even if ya need spaces, that should be server-side...
pub fn process_query(query: Vec<String>) {
    let result = get_search_results(query);

    match result {
        Ok(posts) => run_tui(posts),
        Err(e) => println!("{e}")
    }
}

fn get_search_results(query: Vec<String>) -> Result<Vec<SearchResult>, Error> {
    // TODO: do this on server and send back as an HTTP error
    // And look at the 413 code somewhere to see how you get info from a reqwest error
    // if query.is_empty() {
    //     // NOTE: you could just load the help page
    //     return Err(Error::new("No search query given! Try typing ts --help for info on how to use ts"))
    // }

    let post = reqwest::blocking::get(format!("http://localhost:3000/search?query={}", query.join(" ")))?
    .json::<Vec<SearchResult>>()?;

    return Ok(post);
}