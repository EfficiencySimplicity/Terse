use ratatui::prelude::{Stylize, Span, Line, Text};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Account {
    email: String,
    username: String,
    password: String,
}

impl<'a> From<&'a Account> for Text<'a> {
    fn from(value: &'a Account) -> Self {
        Text::from(vec![
            Line::from(Span::from(&value.username)).red(),
            Line::from(Span::from(&value.password)).blue(),
            Line::from(Span::from(&value.email)).light_blue(),
        ])
    }
}