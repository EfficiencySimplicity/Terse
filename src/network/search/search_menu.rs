use crate::tui::{self, Window};
use crate::network::SearchResults;
use crate::posts::PostWidget;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};


use crossterm::event::KeyCode;

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
                let container = tui::get_default_block().title_bottom("( b to go back )");
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