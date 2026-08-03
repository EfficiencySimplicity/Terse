use ratatui::prelude::{Stylize, Span, Line, Text};
use serde::{Serialize, Deserialize};

use std::fmt::{Display, Write};
use indent_write::fmt::IndentWriter;
use colored::Colorize;

use derive_new::new;

// Should this thing print its password in display?
// I mean if someone's behind you 
// then printing all your passwords is BAD
#[derive(Clone, Serialize, Deserialize, new)]
pub struct Account {
    // You may not have access to the email
    pub email: String,
    pub password: String
}

impl<'a> From<&'a Account> for Text<'a> {
    fn from(value: &'a Account) -> Self {
        Text::from(vec![
            Line::from(Span::from(&value.email)).red(),
            Line::from(Span::from(&value.password)).blue(),
            //Line::from(Span::from(&value.email)).light_blue(),
        ])
    }
}

// Should this even be in-dented?
impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // https://stackoverflow.com/questions/56612060/how-to-call-function-from-certain-trait-explicitly
        write!(f, "{} ({})", Colorize::red(self.email.as_str()), Colorize::blue(self.password.as_str()))?;
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

// TODO: in other cases, I just accept a string as an error,
// not have the server spit out a serde-decodable enum...
// Why do I do it differently here?!