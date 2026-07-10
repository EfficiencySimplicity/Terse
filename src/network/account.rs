use ratatui::prelude::{Stylize, Span, Line, Text};
use serde::{Serialize, Deserialize};

use std::fmt::{Display, Write};
use indent_write::fmt::IndentWriter;

use derive_new::new;

// Should this thing print its password in display?
// I mean if someone's behind you 
// then printing all your passwords is BAD
#[derive(Clone, Serialize, Deserialize, new)]
pub struct Account {
    // You may not have access to the email
    pub email: Option<String>,
    pub username: String,
    pub password: String,
}

impl<'a> From<&'a Account> for Text<'a> {
    fn from(value: &'a Account) -> Self {
        Text::from(vec![
            Line::from(Span::from(&value.username)).red(),
            Line::from(Span::from(&value.password)).blue(),
            //Line::from(Span::from(&value.email)).light_blue(),
        ])
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = IndentWriter::new("\t", f);
        if let Some(email) = &self.email {
            writeln!(i, "email: {}", email)?;
        }
        writeln!(i, "username: {}", self.username)?;
        writeln!(i, "password: {}", self.password)?;
        Ok(())
    }
}

// Why a String message, not an enum with presets?
// Well, people could modify the server code to do whatever.
// Maybe someone has accounts disabled somehow and wants to tell you not to bother;
// This doesn't force the server into any restrictive contracts.
#[derive(Deserialize)]
pub enum AccountCreationMessage {
    Sure,
    Nope(String)
}