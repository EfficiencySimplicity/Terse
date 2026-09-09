use std::sync::Arc;
use parking_lot::RwLock;
use crate::tui::{FramedWindow, Window, Label};
use crate::network::{Server, ServerList};
use crate::posts::Post;

use ratatui::widgets::{StatefulWidget, List, ListState};
use ratatui::text::{Span, Line, Text};
use ratatui::prelude::{Rect, Buffer, Modifier};

use crossterm::event::{KeyCode, KeyEvent};

use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct SearchResultHeader {
    pub title: String,
    pub postid: u16,
}

pub struct SearchResult {
    pub header: SearchResultHeader,
    pub server: Server,
}


impl SearchResult {
    pub fn new(header: SearchResultHeader, server: Server) -> Self {
        Self {header, server}
    }
}

// https://www.reddit.com/r/rust/comments/7zm0j2/intofrom_for_nonconsuming_conversions/
impl<'a> From<&'a SearchResultHeader> for Text<'a> {
    fn from(value: &'a SearchResultHeader) -> Self {
        return Text::from(vec![Line::from(Span::from(&value.title))]);
    }
}

pub struct SearchResults {
    links: Vec<SearchResult>,
    list_state: ListState,
    server_list: Arc<RwLock<ServerList>>
}

impl SearchResults{
    pub fn new(links: Vec<SearchResult>, server_list: Arc<RwLock<ServerList>>) -> Self {
        let mut list_state = ListState::default();
        list_state.select_first();

        Self { links, list_state, server_list }
    }

    pub fn get_selected_article(&self) -> Post {
        // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
        let server_list = self.server_list.read();
        let search_result = self.links.get(self.list_state.selected().unwrap_or(0)).unwrap();
        return server_list.get_post(&search_result.server, search_result.header.postid).unwrap();
    }

    pub fn get_width(&self) -> usize {
        // https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or
        return self.links.iter().map(|x| x.header.title.len()).max().unwrap_or(10)
    }
}

impl Window for SearchResults {
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<(), anyhow::Error> {
        match key.code {
            // TODO: clamp after!
            KeyCode::Char('j') => self.list_state.scroll_down_by(1),
            KeyCode::Char('k') => self.list_state.scroll_up_by(1),
            _ => (),
        }
        Ok(())
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(
            List::new(self.links.iter().map(|x| &x.header))
                // This can also be a style.
                .highlight_style(Modifier::REVERSED)
                .scroll_padding(3),
            area,
            buf,
            &mut self.list_state,
        );
    }
}

impl FramedWindow for SearchResults {
    fn get_labels() -> Vec<String> {
        return vec![
            Label::new("j", "down"),
            Label::new("k", "up"),
            Label::new("enter", "select"),
        ]
    }
}