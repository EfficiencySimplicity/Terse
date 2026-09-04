use crate::tui::{FramedWindow, Window, Label};
use crate::network::Server;
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
    pub post: Option<Post>
}


impl SearchResult {
    pub fn new(header: SearchResultHeader, server: Server) -> Self {
        Self {header, server, post: None}
    }

    pub fn get_post(&mut self) -> Post {
        if self.post.is_none() {
            // I can't do this without a Server
            // and so hm. IT'll have to be in the SearchResults that it stores a list of these things;
            // they can't all reference a Server. I mean they could. . .
            // especially if we get cached ones.
            self.post = Some(self.server.get_post(self.header.postid).unwrap())
        }

        return self.post.as_ref().unwrap().clone()
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
}

impl SearchResults{
    pub fn new(links: Vec<SearchResult>) -> Self {
        let mut list_state = ListState::default();
        list_state.select_first();

        Self { links, list_state }
    }

    pub fn get_selected_article(&mut self) -> Post {
        // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
        self.links.get_mut(self.list_state.selected().unwrap_or(0)).unwrap().get_post()
    }

    pub fn get_width(&self) -> usize {
        // https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or
        return self.links.iter().map(|x| x.header.title.len()).max().unwrap_or(10)
    }
}

impl Window for &mut SearchResults {
    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            // TODO: clamp after!
            KeyCode::Char('j') => self.list_state.scroll_down_by(1),
            KeyCode::Char('k') => self.list_state.scroll_up_by(1),
            _ => (),
        }
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

impl FramedWindow for &mut SearchResults {
    fn get_labels() -> Vec<String> {
        return vec![
            Label::new("j", "down"),
            Label::new("k", "up"),
            Label::new("enter", "select"),
        ]
    }
}