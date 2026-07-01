use crate::{posts::{Post}, tui::get_default_block, network::{Server, search::SearchResult}};
use crate::tui::{Window, PostWidget};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    widgets::{List, ListState, StatefulWidget, Widget},
};

use anyhow::Error;

use crossterm::event::KeyCode;


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

    fn get_selected_article(&self) -> Result<Post, Error> {
        // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
        self.server.get_post(self.links.get(self.list_state.selected().unwrap_or(0)).unwrap().postid)
    }
}

impl<'a> Widget for &mut SearchResults<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let container = get_default_block()
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

pub enum SearchMenuMode {
    Results,
    Answer(PostWidget),
}

pub struct SearchMenu<'a> {
    results: SearchResults<'a>,
    mode: SearchMenuMode,
}

impl<'a> SearchMenu<'a> {
    pub fn new(results: SearchResults<'a>) -> Self {
        Self {results, mode: SearchMenuMode::Results}
    }
}

impl<'a> Widget for &mut SearchMenu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
    
        match &mut self.mode {
            SearchMenuMode::Results => {self.results.render(area, buf)}
            SearchMenuMode::Answer(p) => {
                let container = get_default_block().title_bottom("( b to go back )");
                let inner = container.inner(area);

                container.render(area, buf);
                p.render(inner, buf);
            }
        }
    }
}

impl<'a> Window for SearchMenu<'a> {
    fn handle_key_event(&mut self, key: KeyCode) {
        match &mut self.mode {
            SearchMenuMode::Results => {
                match key {
                    KeyCode::Enter => {
                        self.mode = SearchMenuMode::Answer(
                            PostWidget::new(self.results.get_selected_article().unwrap())
                        )
                    }
                    _ => self.results.handle_key_event(key)
                }
            }
            SearchMenuMode::Answer(a) => {
                match key {
                    KeyCode::Char('b') => {
                        self.mode = SearchMenuMode::Results;
                    }
                    _ => a.handle_key_event(key),
                }
            }
        }
    }
}