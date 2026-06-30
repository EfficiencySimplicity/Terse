use crate::network::{Account, server::{Server, ServerValidityError}};

use directories::ProjectDirs;
use reqwest::StatusCode;
use std::{fs::*, path::PathBuf};
use std::fmt::Display;

use url::Url;

use serde::{Serialize, Deserialize};

use anyhow::Error;

//pub type ServerList = Selectable<Server>;

// TODO: make Server accept Client, so it can be Deserialize, so there's no need for builders.
#[derive(Serialize, Deserialize)]
pub struct ServerListBuilder {
    pub servers: Vec<ServerBuilder>
}

#[derive(Serialize, Deserialize)]
pub struct ServerBuilder {
    pub url: String,
    pub accounts: Vec<Account>,
}

impl From<ServerListBuilder> for ServerList {
    fn from(value: ServerListBuilder) -> Self {
        ServerList { servers: value.servers.into_iter().map(Server::from).collect::<Vec<Server>>()}
    }
        // https://stackoverflow.com/questions/63798662/how-do-i-convert-a-vecresultt-e-to-resultvect-e
        // Ok(ServerList { servers: self.servers.into_iter().map(Server::from).collect::<Result<Vec<Server>, Error>>()? }
}

impl From<ServerBuilder> for Server {
    fn from(value: ServerBuilder) -> Self {
        Self::with_accounts(Url::parse(&value.url).expect("Could not parse server!"), value.accounts)
    }
}

impl Into<ServerBuilder> for &Server {
    fn into(self) -> ServerBuilder {
        ServerBuilder { url: String::from(self.url.as_str()), accounts: self.accounts.clone() }
    }
}

impl Into<ServerListBuilder> for &ServerList {
    fn into(self) -> ServerListBuilder {
        ServerListBuilder { servers: self.servers.iter().map(Into::<ServerBuilder>::into).collect() }
    }
}

pub struct ServerList {
    pub servers: Vec<Server>,
}

impl Display for ServerList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Servers: {}", self.servers.len())?;
        for server in &self.servers {
            writeln!(f, "{}", server)?;
        }
        Ok(())
    }
}

impl ServerList {
    pub fn empty() -> Self {
        return Self { servers: vec![] }
    }
    
    pub fn from_config_file() -> Result<ServerList, Error> {
        let servers_file = Self::get_config_file()?;

        // NOTE: there are notes saying you should use try_exists() sometimes.
        match &mut std::fs::read_to_string(&servers_file) {
            Ok(string) => {
                // NOTE: This could be a std::io::BufReader that wraps f;
                // Would that be any better?
                println!("Reading config file...");
                match string.as_str() {
                    "" => return Ok(Self::empty()),
                    _ => return Ok(serde_json::from_str::<ServerListBuilder>(&string)?.into()),
                }
            }
            Err(e) => {
                println!("Creating file...");
                File::create(&servers_file)?;
                return Self::from_config_file();
            }
        }
    }

    pub fn get_config_file() -> Result<PathBuf, Error> {
        // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
        let dirs = ProjectDirs::from("", "InsanityOnAMachine", "Terse").ok_or(Error::msg("Home directory not found!"))?;
        let data_dir = dirs.data_dir();
        let servers_file = data_dir.join("servers");

        std::fs::create_dir_all(servers_file.parent().ok_or(Error::msg("Could not get path parent"))?)?;
        println!("Got config file: {:?}", servers_file.to_str());
        return Ok(servers_file)
    }

    pub fn add_server(&mut self, url: &str) -> Result<(), AddServerError> {
        let mut server = Server::new(Url::parse(url)?);
        
        if self.servers.iter().any(|x| return x.url == server.url) {
            return Err(AddServerError::ServerAlreadyExists)
        }
        
        if server.exists_and_is_a_terse_server()? == () {
            self.servers.push(server);
            return Ok(self.store()?);
        } else {
            return Err(AddServerError::ServerSaysItIsntATerseServer)
        }
    }

    pub fn store(&self) -> Result<(), Error> {
        Ok(write(&Self::get_config_file()?, serde_json::to_string(&Into::<ServerListBuilder>::into(self))?)?)
    }
}

// https://stackoverflow.com/questions/48430836/rust-proper-error-handling-auto-convert-from-one-error-type-to-another-with-que
#[derive(thiserror::Error, Debug)]
pub enum AddServerError {
    #[error("I wasn't able to parse the url")]
    CantParseUrl,
    #[error("I can't locate data file where the list of servers is stored")]
    CantGetDataFile,
    #[error("Server not responding; status code {0}")]
    ServerNotResponding(String),
    #[error("The server returned a {0} error when accessing the /exists-and-is-a-terse-server page")]
    ServerGaveError(String),
    #[error("The url you gave doesn't lead to an actively running Terse server")]
    ServerNotATerseServer,
    #[error("The url you gave seems to be a Terse server, but it didn't answer with a readable true/false response")]
    ServerGaveBadData,
    #[error("The url you gave went through the effort to actually explicitly say that it isn't a Terse server")]
    ServerSaysItIsntATerseServer,
    #[error("I wasn't able to save the new server to the data file")]
    UnableToSaveServerList,
    #[error("You already have that server saved")]
    ServerAlreadyExists,
    #[error("The server told me to redirect")]
    ServerRedirected,
    #[error("I found an unexpected error: {0}")]
    UnknownError(Error)
}

// https://burntsushi.net/rust-error-handling/#the-from-trait
impl From<url::ParseError> for AddServerError {
    fn from(value: url::ParseError) -> Self {
        Self::CantParseUrl
    }
}

impl From<ServerValidityError> for AddServerError {
    fn from(value: ServerValidityError) -> Self {
        Self::ServerNotATerseServer
    }
}

impl From<Error> for AddServerError {
    fn from(value: Error) -> Self {
        AddServerError::UnableToSaveServerList
    }
}
// Literally window and widget can both be implemented already...