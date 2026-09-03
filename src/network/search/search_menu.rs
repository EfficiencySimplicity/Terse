use ratatui::layout::{ Layout, Direction, Constraint };
use crate::tui::{self, FramedWindow, Window, Label};
use super::SearchResults;
use crate::posts::PostWidget;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};

use crossterm::event::KeyCode;

pub enum SearchMenuMode {
    Results,
    Answer (PostWidget),
}

pub struct SearchMenu<'a> {
    results: SearchResults<'a>,
    mode: SearchMenuMode,
}

impl<'a> SearchMenu<'a> {
    pub fn new(results: SearchResults<'a>) -> Self {
        Self {results: results, mode: SearchMenuMode::Results}
    }
}


impl<'a> Window for SearchMenu<'a> {
    fn handle_key_event(&mut self, key: KeyCode) {
        match &mut self.mode {
            SearchMenuMode::Results => {
                match key {
                    KeyCode::Enter => {
                        self.mode = SearchMenuMode::Answer(PostWidget::new(self.results.get_selected_article()));
                    }
                    _ => (&mut self.results).handle_key_event(key)
                };
            }
            SearchMenuMode::Answer(post_widget) => {
                match key {
                    KeyCode::Char('b') => {
                        self.mode = SearchMenuMode::Results;
                    }
                    _ => post_widget.handle_key_event(key),
                }
            }
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [left, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(std::cmp::max(self.results.get_width() as u16 + 8, 35)),
                Constraint::Fill(1),
            ])
            .areas(area);

        if let SearchMenuMode::Answer(post_widget) = &mut self.mode {
            (&mut self.results).render_unselected(left, buf);
            post_widget.render_selected(right, buf, &mut vec![Label::new("b", "back")]);
        } else {
            (&mut self.results).render_selected(left, buf, &mut vec![]);
            // Later this can be... a Future or an Option or something...
            // that renders even if it has one or not.
            tui::get_default_block().render(right, buf);
        }
        return
    }
}