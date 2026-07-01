use directories::ProjectDirs;
use std::fs::*;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::network::{Server, ServerValidityError};
use url::Url;

use std::fmt::Display;
use indoc::indoc;

use anyhow::Error;


#[derive(Default, Clone, Serialize, Deserialize)]
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
    pub fn selected(&mut self) -> Option<&mut Server> {
        return self.selected.map(|x| &mut self.servers[x as usize])
    }

    pub fn get_config_file() -> Result<PathBuf, SerializationError> {

        // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
        let dirs = ProjectDirs::from("", "InsanityOnAMachine", "Terse").ok_or(SerializationError::BadFilesystemConfig)?;
        let servers_file = dirs.data_dir().join("servers");

        // We try to create the path of folders leading to the servers file;
        // This could be handled by an install script one day so we don't need to check 
        // every time Terse is run
        std::fs::create_dir_all(
            servers_file.parent()
            .expect("The storage file path I tried had no parent, IMPOSSIBLE. Your system is haunted! Scary!")
        ).or(Err(SerializationError::CouldntCreateFile))?;

        return Ok(servers_file)
    }
    
    // TODO: SerializationError::CouldntCreateFile...
    // It shouldn't cause too much trouble but I recently removed some of it's verbosity
    // (see nearest expect() above)
    // And now I see that from_config file tries to create it, and it returns an Error
    // instead of what's useful...

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
                    "" => return Ok(Self::default()),
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

    pub fn add_server(&mut self, url: Url) -> Result<(), AddServerError> {        
        // Sometimes different urls redirect to the same url in the end;
        // I feel like this should be allowed, Terse should just assume
        // that redirects are two different servers.
        if self.servers.iter().any(|x| {x.url == url}) {
            return Err(AddServerError::ServerAlreadyExists)
        }

        let server = Server::new(url);
        
        server.exists_and_is_a_terse_server()?;

        self.servers.push(server);
        // NOTE: maybe always hop onto the brand new server?
        // or maybe a flag to do so or avoid doing so
        if self.servers.len() == 1 { self.selected = Some(0) }
        return Ok(self.store()?);
    }

    pub fn remove_server(&mut self, url: Url) -> Result<(), Error> {
        
        if self.servers.iter().all(|x| {x.url != url}) {
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
    
        self.servers.retain(|x| {x.url != url});
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

#[derive(Serialize, Deserialize)]
pub struct ServerListSerializer {
    pub servers: Vec<Server>,
    pub selected: Option<usize>,
}

impl From<ServerListSerializer> for ServerList {
    fn from(value: ServerListSerializer) -> Self {
        ServerList { servers: value.servers, selected: value.selected }
    }
}

impl Into<ServerListSerializer> for ServerList {
    fn into(self) -> ServerListSerializer {
        ServerListSerializer { servers: self.servers.clone(), selected: self.selected }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SerializationError {
    // https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.from
    #[error("I couldn't decide where to look for the storage file, I couldn't find a $HOME base")]
    BadFilesystemConfig,
    #[error("I couldn't find the storage file, and I couldn't create it and all the parent folders it needed")]
    CouldntCreateFile,
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