use crate::queries::api::*;
use crate::tui::*;
use crate::posts::{Post, get_post};
use reqwest::{Error};
use crate::tui::Window;
use crate::posts::PostWidget;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    style::Style,
    widgets::{Block, BorderType, List, ListState, StatefulWidget, Widget},
};

use crossterm::event::KeyCode;


pub struct SearchResults {
    links: Vec<SearchResult>,
    list_state: ListState,
}

impl SearchResults {
    fn new(links: Vec<SearchResult>) -> Self {
        let mut list_state = ListState::default();
        list_state.select_first();

        Self { links, list_state }
    }

    fn get_selected_article(&self) -> Result<Post, Error> {
        // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
        get_post(self.links.get(self.list_state.selected().unwrap_or(0)).unwrap().postid)
    }
}

impl Widget for &mut SearchResults {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // inner is inside the block, not the whole magic scrollview buffer...

        let container = Block::bordered()
            .border_type(BorderType::HeavyQuadrupleDashed)
            .border_style(Style::new().white().on_white())
            .title_bottom("( (j / k) + enter) or (id) to select");

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

impl Window for SearchResults {
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

pub struct SearchMenu {
    results: SearchResults,
    mode: SearchMenuMode,
}

impl SearchMenu {
    fn new(results: SearchResults) -> Self {
        Self {results, mode: SearchMenuMode::Results}
    }
}

impl Widget for &mut SearchMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match &mut self.mode {
            SearchMenuMode::Results => {self.results.render(area, buf)}
            SearchMenuMode::Answer(p) => p.render(area, buf)
        }
    }
}

impl Window for SearchMenu {
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

pub fn run_tui(results: Vec<SearchResult>) {
    ratatui::run(|terminal| App::default().run(terminal, &mut SearchMenu::new(SearchResults::new(results))));
}