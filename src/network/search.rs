use crate::network::Server;

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
            self.client.get(self.with_params("search", format!("query={}", query.join(" "))))
            .send()?
            .json::<Vec<SearchResult>>()?
        )
    }
}