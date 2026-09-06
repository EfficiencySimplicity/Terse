use std::sync::Arc;
use reqwest::{StatusCode, Client};
use serde::{Serialize, Deserialize};

use colored::Colorize;
use bytesize::ByteSize;
use anyhow::Error;
use url::Url;

use std::fmt::{Display, Formatter, Write};

use crate::data::CLIENT;
use crate::network::{LoginInfo, SearchResult, SearchResultHeader};
use crate::posts::{Post, PublishingResult};

// RIP: use std::borrow::Borrow; I have no idea why you even existed or if I even wrote you.

// https://stackoverflow.com/questions/63369629/how-can-i-split-up-a-large-impl-over-multiple-files

// NOTE: In the end, this should be async so it don't block the TUI
#[derive(Clone, Serialize, Deserialize)]
#[serde(from="ServerSerializer")]
#[serde(into="ServerSerializer")]
pub struct Server {
    #[serde(with = "url_serde")]
    url: Url, 
    login_info: Option<LoginInfo>,
}

impl Server {

    pub fn new(url: Url, login_info: Option<LoginInfo>) -> Self {
        Self {url, login_info}
    }

    pub fn url(&self) -> Url {
        self.url.clone()
    }

    pub fn set_login_info(&mut self, login_info: LoginInfo) {
        self.login_info = Some(login_info)
    }

    pub fn is_signed_in(&self) -> bool {
        self.login_info.is_some()
    }

    pub fn identifier_string(&self) -> String {
        return format!("{} ({})", self.url(), self.get_stats().map_or(String::from("Couldn't get name"), |x| x.server_name))
    }

    pub fn user_string(&self) -> String {
        return format!(
            "{} on {} ({})",
            self.login_info.clone().map_or(String::from("(Not signed in)"), |x| x.email),
            self.url(),
            self.get_stats().map_or(String::from("Couldn't get name"), |x| x.server_name))
    }

    pub fn exists_and_is_a_terse_server(&self) -> Result<(), ServerValidityError> {
        let status = CLIENT.get(self.with_params("exists-and-is-a-terse-server", ""))
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

    // test the server, etc
    pub fn request_login(&self, login_info: &LoginInfo) -> Result<LoginOption, Error> {
        Ok(
            CLIENT.post(self.with_params("user/login", ""))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&login_info)?)
            .send()?
            .json::<LoginOption>()?
        )
    }

    // This, with some changes, could be merged with request_login
    pub fn verify_user(&self, login_info: &LoginInfo, code: &str) -> Result<bool, Error> {
        // Edge cases:
        // - The login info has like, a bad email address ("asfasodfhaoiuf/s.as.ai" or so)
        //     (but this fn is only called after request_login, so it'd be caught then)
        Ok(
            // I don't wanna hafta figure out how to stick the code in the body too.
            CLIENT.post(self.with_params("user/verify", format!("code={code}")))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&login_info)?)
            .send()?
            .json::<bool>()?
        )
    }

    pub fn get_post(&self, id: u16) -> Result<Post, Error> {
        // https://docs.rs/url/latest/url/struct.Url.html#method.parse_with_params
        Ok(
            CLIENT.get(self.with_params("posts", format!("id={id}")))
            .send()?
            .json::<Post>()?
        )
    }

    pub fn publish(&self, post: Post) -> Result<PublishingResult, Error> {
        // https://docs.rs/reqwest/latest/reqwest/blocking/struct.RequestBuilder.html
        
        if !self.is_signed_in() {
            return Err(Error::msg("You aren't signed in to this server, so you cannot publish; try using the    trs --server login    command"))
        }

        // https://stackoverflow.com/questions/499591/are-https-urls-encrypted
        // So I can place the login info within the query! Yippee!
        Ok(
            CLIENT.post(self.with_params("posts", format!("user={}", serde_json::to_string(&self.login_info.clone().unwrap())?)))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&post)?)
            .send()?
            .json::<PublishingResult>()?
        )
    }

    pub fn get_stats(&self) -> Result<ServerStats, Error> {
        Ok(
            CLIENT.get(self.with_params("stats", ""))
            .send()?
            .json::<ServerStats>()?
        )
    }

    pub fn search(&self, query: String) -> Result<Vec<SearchResult>, Error> {
        let headers = CLIENT.get(self.with_params("search", format!("query={}", query)))
            .send()?
            .json::<Vec<SearchResultHeader>>()?;

        Ok(headers.into_iter().map(|x| SearchResult::new(x, self.clone())).collect())
    }

    pub fn as_string(&self, show_password: bool, selected: bool) -> String {
        let mut s = String::new();

        let identifier = self.identifier_string();
        if selected {
            writeln!(s, "{}", Colorize::yellow(identifier.as_str())).expect("String writing should always work");
        } else {
            writeln!(s, "{}", identifier).expect("String writing should always work");
        }
        writeln!(s, "{}", self.login_info.clone().map_or(
            String::from("(Not signed in)"), 
            |x| format!("Signed in as {}", x.as_string(show_password)))
        ).expect("String writing should always work");
        s
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string(false, false))?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub struct ServerSerializer {
    pub url: String,
    pub login_info: Option<LoginInfo>
}

// aw shoot this cant get the client just from deserializing...
impl From<ServerSerializer> for Server {
    fn from(value: ServerSerializer) -> Self {
        Self::new(Url::parse(&value.url).expect("Could not parse server!"), value.login_info)
    }
}

impl Into<ServerSerializer> for Server {
    fn into(self) -> ServerSerializer {
        ServerSerializer { url: String::from(self.url.as_str()), login_info: self.login_info.clone() }
    }
}

#[derive(Deserialize)]
pub struct ServerStats {
    pub server_name: String,
    version: String,
    users: u32,
    posts: u32,
    age: f64,
    max_post_size: u64,
    max_title_size: u64,
}

impl Display for ServerStats {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // TODO: that bookmarked clean-list library
        // TODO: coloration
        writeln!(f, "{} running Terse version {}", self.server_name, self.version)?;
        writeln!(f, "Users on server: {}", self.users)?;
        writeln!(f, "Answers on server: {}", self.posts)?;
        writeln!(f, "Server instance age: {}", self.age)?;
        writeln!(f, "Maximum post size: {}", ByteSize::b(self.max_post_size))?;
        writeln!(f, "Maximum title size: {} characters", self.max_title_size)?;
        Ok(())
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
    fn from(_value: reqwest::Error) -> Self {
        ServerValidityError::CannotConnect
    }
}

// What the server sends back when you ask to login
#[derive(Deserialize)]
pub enum LoginOption {
    PleaseVerify,
    Success,
    BadPassword,
}