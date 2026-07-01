use crate::network::{Server, ServerValidityError};

use directories::ProjectDirs;
use std::{fs::*, path::PathBuf};
use std::fmt::Display;

use indoc::indoc;

use url::Url;

use serde::{Serialize, Deserialize};

use anyhow::Error;


#[derive(Serialize, Deserialize)]
pub struct ServerListSerializer {
    pub servers: Vec<Server>,
    pub selected: Option<usize>,
}

impl From<ServerListSerializer> for ServerList {
    fn from(value: ServerListSerializer) -> Self {
        ServerList { servers: value.servers, selected: value.selected }
    }
        // https://stackoverflow.com/questions/63798662/how-do-i-convert-a-vecresultt-e-to-resultvect-e
        // Ok(ServerList { servers: self.servers.into_iter().map(Server::from).collect::<Result<Vec<Server>, Error>>()? }
}

impl Into<ServerListSerializer> for ServerList {
    fn into(self) -> ServerListSerializer {
        ServerListSerializer { servers: self.servers.clone(), selected: self.selected }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(from="ServerListSerializer")]
#[serde(into="ServerListSerializer")]
pub struct ServerList {
    pub servers: Vec<Server>,
    pub selected: Option<usize>,
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
        return Self { servers: vec![], selected: None }
    }

    pub fn selected(&mut self) -> Option<&mut Server> {
        return self.selected.map(|x| &mut self.servers[x as usize])
    }
    
    pub fn from_config_file() -> Result<ServerList, Error> {
        let servers_file = Self::get_config_file()?;

        // NOTE: there are notes saying you should use try_exists() sometimes.
        match &mut std::fs::read_to_string(&servers_file) {
            Ok(string) => {
                // NOTE: This could be a std::io::BufReader that wraps f;
                // Would that be any better?
                // Also, can serde just read from file?
                println!("Reading config file...");
                match string.as_str() {
                    "" => return Ok(Self::empty()),
                    _ => return Ok(serde_json::from_str::<ServerList>(&string)?.into()),
                }
            }
            Err(_) => {
                println!("Creating file...");
                File::create(&servers_file)?;
                return Self::from_config_file();
            }
        }
    }

    pub fn get_config_file() -> Result<PathBuf, SerializationError> {
        // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
        let dirs = ProjectDirs::from("", "InsanityOnAMachine", "Terse").ok_or(SerializationError::BadFilesystemConfig)?;
        let data_dir = dirs.data_dir();
        let servers_file = data_dir.join("servers");

        std::fs::create_dir_all(
            servers_file.parent()
            .ok_or(SerializationError::CouldntCreateFile(
                Error::msg("The storage file path I tried had no parent, IMPOSSIBLE. Your system is haunted! Scary!")
            ))?
        )
        .or(Err(SerializationError::CouldntCreateFile(
            Error::msg("I couldn't create the path to the storage file")
        )))?;
        return Ok(servers_file)
    }

    pub fn add_server(&mut self, url: &str) -> Result<(), AddServerError> {
        let server = Server::new(Url::parse(url)?);
        
        // Sometimes different urls redirect to the same url in the end;
        // I feel like this should be allowed, Terse should just assume
        // that redirects are two different servers.
        if self.servers.iter().any(|x| {x.url == server.url}) {
            return Err(AddServerError::ServerAlreadyExists)
        }
        
        server.exists_and_is_a_terse_server()?;

        self.servers.push(server);
        // NOTE: maybe always hop onto the brand new server?
        // or maybe a flag to do so or avoid doing so
        if self.servers.len() == 1 { self.selected = Some(0) }
        return Ok(self.store()?);
    }

    pub fn remove_server(&mut self, url: &str) -> Result<(), Error> {

        // We parse the URL 'cause it seems to do a *bit* of normalization
        let server = Server::new(Url::parse(url)?);
        
        if self.servers.iter().all(|x| {x.url != server.url}) {
            return Err(Error::msg(
                format!(
                    indoc! {
                        "I couldn't find a server named {} in the list; 
                        Try running --server list to see all the servers you have",
                    },
                    url
                )
            ))
        }
    
        self.servers.retain(|x| {x.url != server.url});
        if self.servers.len() == 0 { self.selected = None } else
        if self.selected.expect("If the length of the server list is non-zero, the selected element should be Some") 
        > self.servers.len() - 1 { self.selected = Some(self.servers.len() - 1) }
        return Ok(self.store()?);
    }

    pub fn store(&self) -> Result<(), SerializationError> {
        // TODO: serde_json::to_writer takes a Write-able; try using it?
        let storage_file = Self::get_config_file()?;
        let json_string = serde_json::to_string(self).or(Err(SerializationError::CantSerializeSelf))?;

        write(storage_file, json_string).or(Err(SerializationError::CantWriteToFile))?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SerializationError {
    // https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.from
    #[error("I couldn't decide where to look for the storage file, I couldn't find a $HOME base")]
    BadFilesystemConfig,
    #[error("I couldn't find the storage file, and I had trouble creating it:\n{0}")]
    CouldntCreateFile(Error),
    #[error("I tried to read the storage file, but it contained bad data")]
    BadDataInFile,
    #[error("I couldn't successfully turn the data into JSON to save it in the storage file.\nThis shouldn't ever happen, lucky you!")]
    CantSerializeSelf,
    #[error("I couldn't save to the storage file")]
    CantWriteToFile,
}

// https://stackoverflow.com/questions/48430836/rust-proper-error-handling-auto-convert-from-one-error-type-to-another-with-que
#[derive(thiserror::Error, Debug)]
pub enum AddServerError {
    #[error("I wasn't able to parse the url")]
    CantParseUrl,
    #[error("I had a problem with storage;\n{0}")]
    SerializationError(SerializationError),
    #[error("You already have that server saved")]
    ServerAlreadyExists,
    #[error("I wasn't able to add the server;\n{0}")]
    Other(Error)
}

// https://burntsushi.net/rust-error-handling/#the-from-trait
impl From<url::ParseError> for AddServerError {
    fn from(value: url::ParseError) -> Self {
        Self::CantParseUrl
    }
}

impl From<ServerValidityError> for AddServerError {
    fn from(value: ServerValidityError) -> Self {
        Self::Other(Error::from(value))
    }
}

impl From<SerializationError> for AddServerError {
    fn from(value: SerializationError) -> Self {
        Self::SerializationError(value)
    }
}
// Literally window and widget can both be implemented already...