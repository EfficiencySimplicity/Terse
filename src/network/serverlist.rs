use directories::ProjectDirs;
use std::fs::*;
use std::path::PathBuf;
use std::fmt::Write;

use serde::{Serialize, Deserialize};

use crate::network::Server;
use url::Url;

use anyhow::Error;


#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(from="ServerListSerializer")]
#[serde(into="ServerListSerializer")]
pub struct ServerList {
    pub servers: Vec<Server>,
    selected: Option<usize>,
}


impl ServerList {
    pub fn selected(&mut self) -> Result<&mut Server, SelectedServerError> {
        // This should explicitly check for no servers...
        self.selected.map(|x| &mut self.servers[x as usize]).ok_or(SelectedServerError::NoServers)
    }
    
    // For when you need the selected server but ain't gonna modify it.
    pub fn clone_selected(&self) -> Result<Server, SelectedServerError> {
        Ok(self.selected.map(|x| self.servers[x as usize].clone()).ok_or(SelectedServerError::NoServers)?)
    }

    // If this returns Ok, the storage file is guaranteed to exist
    pub fn get_config_file() -> Result<PathBuf, SerializationError> {

        let servers_file;
        // We store to a different data file on debug builds vs. release builds
        // https://stackoverflow.com/questions/39204908/how-to-check-release-debug-builds-using-cfg-in-rust
        if cfg!(not(debug_assertions)) {
            // // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
            let dirs = ProjectDirs::from("", "InsanityOnAMachine", "Terse")
            .ok_or(SerializationError::BadFilesystemConfig)?;
            servers_file = dirs.data_dir().join("servers.json");
        } else {
            servers_file = PathBuf::from("./data/servers.json")
        }

        // https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.exists
        // Here I opt to NOT use try_exists, because if I can't verify it exists,
        // I probably can't access it either.
        // This could be handled by an install script one day so we don't need to check all the time
        if !servers_file.exists() {

            if let Some(folder_path) = servers_file.parent() {
                std::fs::create_dir_all(folder_path)
                .or(Err(SerializationError::CouldntCreateFile))?;
            }

            File::create(&servers_file)
            .or(Err(SerializationError::CouldntCreateFile))?;
        }

        return Ok(servers_file)
    }

    pub fn from_config_file() -> Result<ServerList, Error> {
        let servers_file = Self::get_config_file()?;

        // NOTE: This could be a std::io::BufReader that wraps f;
        // Would that be any better?
        // Also, can serde just read from file?

        let string = std::fs::read_to_string(&servers_file)?;

        match string.as_str() {
            "" => return Ok(Self::default()),
            _  => return Ok(serde_json::from_str::<ServerList>(&string)?.into()),
        }
    }

    pub fn store(&self) -> Result<(), SerializationError> {
        // TODO: serde_json::to_writer takes a Write-able; try using it?
        let storage_file = Self::get_config_file()?;
        let json_string = serde_json::to_string(self).or(Err(SerializationError::CantSerializeSelf))?;

        write(storage_file, json_string).or(Err(SerializationError::CantWriteToFile))?;
        Ok(())
    }

    pub fn add_server(&mut self, url: Url) -> Result<(), Error> {

        // Sometimes different urls redirect to the same url in the end;
        // I feel like this should be allowed, Terse should just assume
        // that redirects are two different servers. For whatever reason.
        if self.servers.iter().any(|x| {x.url() == url}) {
            Err(AddServerError::ServerAlreadyExists)?
        }

        let server = Server::new(url, None);
        
        server.exists_and_is_a_terse_server()?;

        self.servers.push(server);
        // NOTE: maybe always hop onto the brand new server?
        // or maybe a flag to do so or avoid doing so
        if self.servers.len() == 1 { self.selected = Some(0) }
        Ok(self.store()?)
    }

    pub fn remove_server(&mut self, url: Url) -> Result<(), Error> {
        
        if self.servers.iter().all(|x| {x.url() != url}) {
            Err(RemoveServerError::ServerNotInList)?
        }
    
        self.servers.retain(|x| {x.url() != url});

        if self.servers.len() == 0 { self.selected = None } else
        if self.selected.expect("The selected element should be Some, since the length of the server list is non-zero") 
        > self.servers.len() - 1 { self.selected = Some(self.servers.len() - 1) }

        Ok(self.store()?)
    }

    // NOTE: maybe return the selected server?
    pub fn set_server(&mut self, idx: usize) -> Result<&mut Server, Error> {
        if self.servers.is_empty() {
            Err(SelectedServerError::NoServers)?
        } else if idx >= self.servers.len() {
            Err(SelectedServerError::OutOfBounds{ idx, max: self.servers.len() - 1})?
        } else {
            self.selected = Some(idx);
            self.store()?;
            Ok(self.selected().expect("This should point to a server, of course!"))
        }
    }

    pub fn as_string(&self, show_passwords: bool) -> String {
        let mut s = String::new();
        writeln!(s, "Servers: {}", self.servers.len()).expect("String writing should always work");
        if self.servers.is_empty() {return s}
        
        writeln!(s, "Current server: {}", 
            self.clone_selected()
            .expect("Selected should be ok; there is at least 1 server")
            .as_string(show_passwords, true)
        ).expect("String writing should always work");

        for (i, server) in self.servers.iter().enumerate() {
            writeln!(s, "{i}: {}", server.as_string(show_passwords, self.selected == Some(i))).expect("String writing should always work");
        }

        s
    }

}

impl std::fmt::Display for ServerList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.as_string(false))?;
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
pub enum SelectedServerError {
    #[error("You don't have any servers; add one with `trs --server add https://url-to-a-terse-server`")]
    NoServers,
    // max should be servers.len() - 1; the max index available
    #[error("You don't have a server with index {idx}; try an index from 0-{max}")]
    OutOfBounds {idx: usize, max: usize}
}

#[derive(thiserror::Error, Debug)]
pub enum SerializationError {
    // https://docs.rs/directories/latest/directories/struct.ProjectDirs.html#method.from
    #[error("I couldn't decide where to look for the storage file, I couldn't find a $HOME base")]
    BadFilesystemConfig,
    #[error("I couldn't find the storage file, and I couldn't create a new file at the path where it should be")]
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
    #[error("You already have that server saved")]
    ServerAlreadyExists,
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveServerError {
    #[error("I couldn't find the server you wanted to remove in the list;\nTry running --server list to see all the servers you have")]
    ServerNotInList
}