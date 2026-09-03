use crate::tui;

use ratatui::prelude::{Widget, Buffer, Rect};
use crossterm::event::KeyCode;

pub trait Window {
    fn handle_key_event(&mut self, key: KeyCode);
    fn render_contents(&mut self, area: Rect, buf: &mut Buffer);
    fn render(&mut self, area: Rect, buf: &mut Buffer) {self.render_with_labels(area, buf, &mut vec![])}
    fn render_with_labels(&mut self, area: Rect, buf: &mut Buffer, labels: &mut Vec<String>) {
        labels.append(&mut Self::get_labels());
        let block = tui::get_default_block()
            .title_bottom(labels.join("-"));
        
        let inner = block.inner(area);
        block.render(area, buf);
        self.render_contents(inner, buf);
    }
    // this could be in render()
    fn get_labels() -> Vec<String> {vec![]}
}