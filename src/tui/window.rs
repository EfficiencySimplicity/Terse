use ratatui::layout::{Offset, Size};
use ratatui::text::Text;
use crate::tui::{self, App};

use ratatui::prelude::{Widget, Buffer, Rect};
use ratatui::widgets::Block;
use ratatui::style::{Style, Stylize};
use crossterm::event::KeyEvent;

pub trait Window {
    fn handle_key_event(&mut self, key: KeyEvent);
    fn render(&mut self, area: Rect, buf: &mut Buffer);
}

pub trait FramedWindow: Window {
    fn render_selected(&mut self, area: Rect, buf: &mut Buffer, labels: &mut Vec<String>) {
        labels.append(&mut Self::get_labels());
        self.render_in_block(area, buf, tui::get_default_block()
            .title_bottom(labels.join("-")));
    }

    fn render_unselected(&mut self, area: Rect, buf: &mut Buffer, message: &(impl AsRef<str> + ?Sized)) {
        self.render_in_block(area, buf, tui::get_default_block());
        buf.set_style(area, Style::new().gray());

        Text::from(message.as_ref()).light_red().render(area.offset(Offset {x: 1, y: 0}).resize(Size {width: message.as_ref().len() as u16, height: 1}), buf)
    }

    fn render_in_block(&mut self, area: Rect, buf: &mut Buffer, block: Block) {
        let inner = block.inner(area);
        block.render(area, buf);
        self.render(inner, buf);
    }

    fn get_labels() -> Vec<String> {vec![]}
}