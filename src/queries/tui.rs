use crate::queries::api::*;
use crate::tui::*;

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

impl SearchResults {
    fn new(links: Vec<SearchResult>) -> Self {
        let mut list_state = ListState::default();
        list_state.select_first();

        Self { links, list_state }
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

pub fn run_tui(results: Vec<SearchResult>) {
    ratatui::run(|terminal| App::default().run(terminal, &mut SearchResults::new(results)));
}