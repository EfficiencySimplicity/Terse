use ratatui::{
    DefaultTerminal,
    widgets::Widget,
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

#[derive(Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn run<T: Window>(&mut self, terminal: &mut DefaultTerminal, window: &mut T) -> Result<(), std::io::Error> where for<'a> &'a mut T: Widget {
        while !self.exit {
            terminal.draw(|frame| {
                // https://stackoverflow.com/questions/30026893/how-to-use-a-map-over-vectors#30026986
                window.render(frame.area(), frame.buffer_mut());
            })?;

            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Esc => self.exit = true,
                        _ => window.handle_key_event(key_event.code),
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }
}

pub trait Window {
    fn handle_key_event(&mut self, key: KeyCode);
}