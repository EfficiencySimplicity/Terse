// https://ratatui.rs/tutorials/counter-app/basic-app/

use std::io;

// We don't import * because some things have the same name
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
// Wow, you can have quite diverse paths *inside* a use block!
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame
};

// Default (learned it just now) is helpful! I hope partial defaults wexist!
#[derive(Debug, Default)]
pub struct App {
    counter: u8,// ooh efficient memory and a possible error demonstration!
    exit: bool,// extra comma for safety. This is the mark of a wise dev,
    // . . .meaning I never do it.
}

impl App {

    pub fn run(&mut self, terminal: &mut DefaultTerminal) ->  io::Result<()> {
        // This is a bool, stored in the App struct
        while !self.exit {
            // You call terminal.draw with a thing that accepts a Frame.
            // This must render the entire UI.
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    // A Frame is, I suppose, a thing we draw to, and hopefully we use up 60 a second.
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        // This uses blocking, so any actual non-UI *computation* should pass a fn
        // to event::poll() instead, with a good timeout duration 
        match event::read()? {
            // Blah blah blah make sure the event is a press 'cause on Windows it also streams releases / repeats
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }

            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            // Rust enums were MADE for key value modelling
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    // These below could naturally cause overflow panicking (good that it panics, tho!)
    // We could use saturating arithmetic to have it clamped, ideally

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}

// Why borrowed? Why separate? Why.
impl Widget for &App {
    // Rect rendering in a text field? Weird...
    fn render(self, area: Rect, buf: &mut Buffer) {
        // What is a line, and how did Ratatui add a function to the &str type?
        // Answer: using Traits. It's that simple. But maybe not always ideal.
        let title = Line::from(" Counter App Tutorial ".bold());

        let instructions = Line::from(vec![
            // "converts this type into the (usually inferred) input type"
            // Eh?
            " Decrement ".into(),
            // I assume these call into() under the hood
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        // A dense styling pipeline!!!
        let block = Block::bordered()
        // So we can center styled text, etc etc etc...
        .title(title.centered())
        // This applies to the Block, not the title we just added
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

        // what is Text vs Line vs "the (usually inferred) input type"?
        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            // This, recall, is a u8
            self.counter.to_string().yellow(),
        ])]);

        // Paragraph displays text, and the block(), I assume, borders the text...
        // On hovering over the '.block(block)', it does!
        Paragraph::new(counter_text)
        .centered()
        .block(block)
        .render(area, buf);

    }
}

#[cfg(test)]
mod tests {
    // Apparently we must import super, but have access to all the above imports like Buffer
    // ... because they're imported into super. Makes sense.
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        // ah, the handy #[derive(Default)]
        let app = App::default();
        // A small virtual window
        // I misnamed the buf as bif. Ah well, let's keep it.
        let mut bif = Buffer::empty(Rect::new(0,0,50,4));

        app.render(bif.area, &mut bif);

        // Exactly 50 wide; I checked!
        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);

        // And the style must match also, we're matching bif (which had data rendered
        // to it by app.render) with a manually created Buffer

        // Another way to manage styles, besides applying straight to strings?...
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();

        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(bif, expected);
    }

    #[test]
    fn handle_key_event() {
        let mut app = App::default();

        // into() seems to be a built-in that tries to convert an arg
        // into what the function accepts. Interesting
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
    }

    #[test]
    fn run() {
        let mut app = App::default();
        ratatui::run(|terminal| App::default().run(terminal));
    }
}