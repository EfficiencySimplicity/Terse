use ratatui::{layout::{Constraint, Layout, Rect}, buffer::Buffer, widgets::{StatefulWidget, Widget, Scrollbar, ScrollbarOrientation, ScrollbarState, Paragraph}};
use crossterm::event::KeyCode;
use crate::tui::{Window, get_default_block};
use crate::posts::{Post};

pub struct PostWidget {
    post: Post,
    scroll_state: ScrollbarState,
}

impl PostWidget {
    pub fn new(post: Post) -> Self {
        Self {post, scroll_state: ScrollbarState::new(100)}
    }
}

impl Widget for &mut PostWidget {
    // TODO: something with Margin? see
    // https://ratatui.rs/examples/widgets/scrollbar/
    // TODO: all these in the same block! Scrollbar as part of it!
    // Scrollbar styyyyling!
    // TODO: PostWidget by itself! Organize!
    // TODO: Scrollbar actually to scale!
    fn render(self, area: Rect, buf: &mut Buffer) {

        let layout = Layout::horizontal(vec![
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(area);

        // TODO: eliminate this clone() by any means necessary.
        Paragraph::new(self.post.content.clone())
        .block(get_default_block().title_bottom("( j / k to scroll )"))
        .scroll((self.scroll_state.get_position() as u16, 0))
        .render(layout[0], buf);

        StatefulWidget::render(Scrollbar::new(ScrollbarOrientation::VerticalRight), layout[1], buf, &mut self.scroll_state);
    }
}

impl Window for PostWidget {
    fn handle_key_event(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('j') => {self.scroll_state.next()}
            KeyCode::Char('k') => {self.scroll_state.prev()}
            _ => {}
        }
    }
}