use crate::tui::{self, Window};
use crate::network::Server;
use crate::posts::Post;

use ratatui::widgets::{Widget, StatefulWidget, List, ListState};
use ratatui::text::{Span, Line, Text};
use ratatui::prelude::{Rect, Buffer, Modifier};

use crossterm::event::KeyCode;

use serde::Deserialize;

use anyhow::Error;

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

pub struct SearchResults<'a> {
    server: &'a Server,
    links: Vec<SearchResult>,
    list_state: ListState,
}

impl<'a> SearchResults<'a> {
    pub fn new(server: &'a Server, links: Vec<SearchResult>) -> Self {
        let mut list_state = ListState::default();
        list_state.select_first();

        Self { server, links, list_state }
    }

    pub fn get_selected_article(&self) -> Result<Post, Error> {
        // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
        self.server.get_post(self.links.get(self.list_state.selected().unwrap_or(0)).unwrap().postid)
    }
}

impl<'a> Widget for &mut SearchResults<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let container = tui::get_default_block()
            .title_bottom("( ((j / k) + enter) or (id) to select )");

        StatefulWidget::render(
            List::new(&self.links)
                // This can also be a style.
                .highlight_style(Modifier::REVERSED)
                .scroll_padding(3)
                .block(container),
            area,
            buf,
            &mut self.list_state,
        );
    }
}

impl<'a> Window for SearchResults<'a> {
    fn handle_key_event(&mut self, key: KeyCode) {
        match key {
            // TODO: clamp after!
            KeyCode::Char('j') => self.list_state.scroll_down_by(1),
            KeyCode::Char('k') => self.list_state.scroll_up_by(1),
            _ => (),
        }
    }
}