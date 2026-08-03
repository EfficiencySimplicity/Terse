use ratatui::prelude::{Stylize, Span, Line, Text};
use serde::{Serialize, Deserialize};

use std::fmt::{Display};
use colored::Colorize;

use derive_new::new;

#[derive(Clone, Serialize, Deserialize, new)]
pub struct Account {
    pub email: String,
    pub password: String
}

impl<'a> From<&'a Account> for Text<'a> {
    fn from(value: &'a Account) -> Self {
        Text::from(vec![
            Line::from(Span::from(&value.email)).red(),
            Line::from(Span::from(&value.password)).blue(),
        ])
    }
}

// Should this thing print its password in display?
// I mean if someone's behind you 
// then printing all your passwords is BAD
// But they have pretty colors, so I DO
impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // https://stackoverflow.com/questions/56612060/how-to-call-function-from-certain-trait-explicitly
        write!(f, "{} ({})", Colorize::red(self.email.as_str()), Colorize::blue(self.password.as_str()))?;
        Ok(())
    }
}