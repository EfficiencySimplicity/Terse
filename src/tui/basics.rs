use ratatui::{
    DefaultTerminal,
    widgets::{Widget, Block, BorderType},
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

#[derive(Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn run<T: Window>(&mut self, window: &mut T) -> Result<(), std::io::Error> where for<'a> &'a mut T: Widget {
        ratatui::run(|terminal| self.run_loop(terminal, window))
    }

    pub fn run_loop<T: Window>(&mut self, terminal: &mut DefaultTerminal, window: &mut T) -> Result<(), std::io::Error> where for<'a> &'a mut T: Widget {
        while !self.exit {
            terminal.draw(|frame| {
                // https://stackoverflow.com/questions/30026893/how-to-use-a-map-over-vectors#30026986
                window.render(frame.area(), frame.buffer_mut());
            })?;

            if let Event::Key(key_event) = event::read()? {
                if let KeyEventKind::Press = key_event.kind {
                    match key_event.code {
                        KeyCode::Esc => self.exit = true,
                        other => window.handle_key_event(other),
                    }
                }
            }
        }
        Ok(())
    }
}

pub trait Window {
    fn handle_key_event(&mut self, key: KeyCode);
}

pub fn get_default_block<'a>() -> Block<'a> {
    return Block::bordered()
        .border_type(BorderType::Rounded)
        //.border_style(Style::new().white().on_white())
}