use crate::server::Server;

use serde::Deserialize;
use anyhow::Error;

use ratatui::text::{Span, Line, Text};

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

impl Server {
    pub fn search(&self, query: Vec<String>) -> Result<Vec<SearchResult>, Error> {
        Ok(
            reqwest::blocking::get(
                // Maybe not the most efficient, re-parsing, but it's negligible
                // (Because url::Urls ain't mut, or don't seem to be)
                self.with_params([("query", query.join(" "))])?
            )?
            .json::<Vec<SearchResult>>()?
        )
    }
}