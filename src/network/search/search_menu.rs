use ratatui::layout::{ Layout, Direction, Constraint };
use crate::{data, tui::{self, App, FramedWindow, Label, Window}};
use super::{SearchResults, SearchBar};
use crate::posts::PostWidget;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub enum SearchMenuMode {
    Results,
    Answer (PostWidget),
    Search,
}

pub struct SearchMenu {
    results: SearchResults,
    search_bar: SearchBar,
    mode: SearchMenuMode,
}

// TODO: it should *create* a search menu and prompt *it* to search...
impl SearchMenu {
    pub fn new(query: String, results: SearchResults) -> Self {
        Self {results, search_bar: SearchBar::new(query), mode: SearchMenuMode::Results}
    }
}


impl Window for SearchMenu {
    fn handle_key_event(&mut self, key: KeyEvent) {
        match &self.mode {
            SearchMenuMode::Results | SearchMenuMode::Answer(_) => {
                if let KeyCode::Char('k') = key.code && key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.mode = SearchMenuMode::Search;
                    return
                }
            }
            SearchMenuMode::Search => {
                if let KeyCode::Char('j') = key.code && key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.mode = SearchMenuMode::Results;
                    return
                }
            }
        }

        match &mut self.mode {
            SearchMenuMode::Results => {
                match key.code {
                    KeyCode::Enter => {
                        self.mode = SearchMenuMode::Answer(PostWidget::new(self.results.get_selected_article()));
                    }
                    _ => (&mut self.results).handle_key_event(key)
                };
            }
            SearchMenuMode::Answer(post_widget) => {
                match key.code {
                    KeyCode::Char('b') => {
                        self.mode = SearchMenuMode::Results;
                    }
                    _ => post_widget.handle_key_event(key),
                }
            }
            SearchMenuMode::Search => {
                match key.code {
                    KeyCode::Enter => {
                        let server_list = data::SERVER_LIST.lock();
                        let server = server_list.get_default().unwrap();
                        let results = server.search(self.search_bar.text.clone()).unwrap();
                        self.results = SearchResults::new(results);
                        self.mode = SearchMenuMode::Results;
                    }
                    _ => {}
                }
                self.search_bar.handle_key_event(key);
            }
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [top, bottom] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .areas(area);

        let [left, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(std::cmp::max(self.results.get_width() as u16 + 8, 35)),
                Constraint::Fill(1),
            ])
            .areas(bottom);

        if let SearchMenuMode::Answer(post_widget) = &mut self.mode {
            (&mut self.search_bar).render_unselected(top, buf, "ctrl+k");
            (&mut self.results).render_unselected(left, buf, "b");
            post_widget.render_selected(right, buf, &mut vec![]);
        } else if let SearchMenuMode::Results = &mut self.mode {
            (&mut self.search_bar).render_unselected(top, buf, "ctrl+k");
            (&mut self.results).render_selected(left, buf, &mut vec![]);
            // Later this can be... a Future or an Option or something...
            // that renders even if it has one or not.
            tui::get_default_block().render(right, buf);
        } else {
            (&mut self.search_bar).render_selected(top, buf, &mut vec![Label::new("enter", "search")]);
            (&mut self.results).render_unselected(left, buf, "ctrl+j");
            // Later this can be... a Future or an Option or something...
            // that renders even if it has one or not.
            tui::get_default_block().render(right, buf);
        }
        return
    }
}