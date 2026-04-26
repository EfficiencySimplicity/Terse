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

// The IO::Result is a specialized Result for IO operations
fn main() -> io::Result<()> {
    //AAAAAH a closure! What do we dooooooo?
    ratatui::run(|terminal| App::default().run(terminal))
}