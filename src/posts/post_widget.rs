use ratatui::{layout::{Constraint, Layout, Rect}, buffer::Buffer, widgets::{StatefulWidget, Widget, Scrollbar, ScrollbarOrientation, ScrollbarState, Paragraph}};
use crossterm::event::KeyCode;
use crate::tui::{self, Window, FramedWindow, Label};
use super::Post;

pub struct PostWidget {
    post: Post,
    scroll_state: ScrollbarState,
}

impl PostWidget {
    pub fn new(post: Post) -> Self {
        Self {post, scroll_state: ScrollbarState::new(100)}
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

    // TODO: something with Margin? see
    // https://ratatui.rs/examples/widgets/scrollbar/
    // TODO: all these in the same block! Scrollbar as part of it!
    // Scrollbar styyyyling!
    // TODO: PostWidget by itself! Organize!
    // TODO: Scrollbar actually to scale!
    fn render(&mut self, area: Rect, buf: &mut Buffer) {

        let layout = Layout::horizontal(vec![
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(area);

        // TODO: eliminate this clone() by any means necessary.
        Paragraph::new(self.post.content.clone())
        .scroll((self.scroll_state.get_position() as u16, 0))
        .render(layout[0], buf);

        StatefulWidget::render(Scrollbar::new(ScrollbarOrientation::VerticalRight), layout[1], buf, &mut self.scroll_state);
    }
}

impl FramedWindow for PostWidget {
    fn get_labels() -> Vec<String> {
        return vec![
            Label::new("j", "down"),
            Label::new("k", "up"),
        ]
    }
}