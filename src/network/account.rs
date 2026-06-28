use ratatui::prelude::{Stylize, Span, Line, Text};
use serde::{Serialize, Deserialize};

use std::fmt::{Display, Write};
use indent_write::fmt::IndentWriter;

#[derive(Clone, Serialize, Deserialize)]
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

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = IndentWriter::new("\t", f);
        writeln!(i, "email: {}", self.email)?;
        writeln!(i, "username: {}", self.username)?;
        writeln!(i, "password: {}", self.password)?;
        Ok(())
    }
}