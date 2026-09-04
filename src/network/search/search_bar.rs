use ratatui::text::Line;
use crate::tui::{FramedWindow, Window, App};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
};

use crossterm::event::{KeyCode, KeyEvent};

pub struct SearchBar {
	pub text: String,
}

impl SearchBar {
	pub fn new(query: String) -> Self {
		SearchBar {text: query}
	}
}

impl Window for SearchBar {
	fn handle_key_event(&mut self, key: KeyEvent) {
		match key.code {
            KeyCode::Char(c) => {
                self.text.push(c);
            }
            KeyCode::Backspace => {
                let _ = self.text.pop();
            }
            _ => {}
        }
	}

	fn render(&mut self, area: Rect, buf: &mut Buffer) {
		Line::from(self.text.as_str()).render(area, buf);
	}
}

impl FramedWindow for SearchBar {}