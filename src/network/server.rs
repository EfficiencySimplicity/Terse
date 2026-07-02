use reqwest::{StatusCode, blocking::{Client, Response}};
use anyhow::Error;
use url::Url;
use std::fmt::{Display, Formatter, Write};
use bytesize::ByteSize;

use crate::network::Account;
use indent_write::fmt::IndentWriter;

// RIP: use std::borrow::Borrow; I have no idea why you even existed or if I even wrote you.

use serde::{Serialize, Deserialize};

use crate::posts::Post;

// https://stackoverflow.com/questions/63369629/how-can-i-split-up-a-large-impl-over-multiple-files
pub mod search;
pub use search::*;


// NOTE: In the end, this should be async so it don't block the TUI
#[derive(Clone, Serialize, Deserialize)]
#[serde(from="ServerSerializer")]
#[serde(into="ServerSerializer")]
pub struct Server {   
    pub(crate) url: Url, 
    client: Client,
    accounts: Vec<Account>,
}

impl Server {

    pub fn new(url: Url) -> Self {
        Self {url, client: Client::new(), accounts: vec![]}
    }
    
    pub fn with_accounts(url: Url, accounts: Vec<Account>) -> Self {
        Self {url, client: Client::new(), accounts: accounts}
    }

    pub fn exists_and_is_a_terse_server(&self) -> Result<(), ServerValidityError> {
        let status = self.client.get(self.with_params("exists-and-is-a-terse-server", ""))
        .send()?.status();

        match status {
            StatusCode::OK => Ok(()),
            StatusCode::NOT_FOUND => Err(ServerValidityError::ServerIsNotATerseServer),
            _ => Err(ServerValidityError::Other(status))
        }
    }

    // I opt to use &strs instead of Options as the arguments, although setting query = None is really fun...
    // But it'd be a lotta extra text that could just be an empty &str
    // https://stackoverflow.com/questions/55079070/how-to-accept-str-string-and-string-in-a-single-function
    pub(super) fn with_params(&self, route: &str, query: impl AsRef<str>) -> Url
    {
        let mut url = self.url.clone();
        url.set_path(route);
        url.set_query(Some(query.as_ref()));
        return url;
    }

    pub fn get_post(&self, id: u16) -> Result<Post, Error> {
        // https://docs.rs/url/latest/url/struct.Url.html#method.parse_with_params
        Ok(
            self.client.get(self.with_params("posts", format!("id={id}")))
            .send()?
            .json::<Post>()?
        )
    }

    pub fn publish(&self, post: Post) -> Result<Response, Error> {
        // https://docs.rs/reqwest/latest/reqwest/blocking/struct.RequestBuilder.html
        Ok(
            self.client.post(self.with_params("posts", ""))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&post)?)
            .send()?
        )

        // match result {
        //     // https://docs.rs/reqwest/latest/reqwest/struct.StatusCode.html
        //     Ok(response) => match response.status().as_u16() {
        //         413 => {
        //             let err = response.json::<PostSizeExceptionError>()?;
        //             println!("{err}");
        //         }
        //         _ => println!("Answer published successfully"),
        //     }
        //     Err(err) => println!("Error in sending post: {err}")
        // }
        // Ok(())
    }

    pub fn get_stats(&self) -> Result<ServerStats, Error> {
        Ok(
            self.client.get(self.with_params("stats", ""))
            .send()?
            .json::<ServerStats>()?
        )
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = IndentWriter::new("\t", f);
        writeln!(i, "{}", self.url.as_str())?;
        writeln!(i, "Accounts: {}", self.accounts.len())?;
        for account in &self.accounts {
            writeln!(i, "{}", account)?;
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub struct ServerSerializer {
    pub url: String,
    pub accounts: Vec<Account>,
}

impl From<ServerSerializer> for Server {
    fn from(value: ServerSerializer) -> Self {
        Self::with_accounts(Url::parse(&value.url).expect("Could not parse server!"), value.accounts)
    }
}

impl Into<ServerSerializer> for Server {
    fn into(self) -> ServerSerializer {
        ServerSerializer { url: String::from(self.url.as_str()), accounts: self.accounts.clone() }
    }
}

#[derive(Deserialize)]
pub struct ServerStats {
    version: String,
    users: u32,
    posts: u32,
    age: f64,
    max_post_size: u64,
}

impl Display for ServerStats {

    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // TODO: that bookmarked clean-list library
        // TODO: coloration
        writeln!(f, "Server running Terse version {}", self.version)?;
        writeln!(f, "Users on server: {}", self.users)?;
        writeln!(f, "Answers on server: {}", self.posts)?;
        writeln!(f, "Server instance age: {}", self.age)?;
        writeln!(f, "Maximum post size: {}", ByteSize::b(self.max_post_size))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServerValidityError {
    // No contact at all; the site doesn't exist
    #[error("I couldn't connect to the server")]
    CannotConnect,
    // There's no /exists-and-is-a-terse-server path
    #[error("The server isn't a Terse server")]
    ServerIsNotATerseServer,
    // Any other status code
    #[error("The server returned an unexpected status code: {0}")]
    Other(StatusCode),
}

impl From<reqwest::Error> for ServerValidityError {
    fn from(value: reqwest::Error) -> Self {
        ServerValidityError::CannotConnect
    }
}