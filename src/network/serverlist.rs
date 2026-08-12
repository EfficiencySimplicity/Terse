use std::fs::*;
use std::path::PathBuf;
use std::fmt::Write;

use serde::{Serialize, Deserialize};

use crate::network::Server;
use crate::data::DataStorageError;
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

    pub fn from_config_file(file_path: PathBuf) -> Result<ServerList, DataStorageError> {

        // NOTE: This could be a std::io::BufReader that wraps f;
        // Would that be any better?
        // Also, can serde just read from file?

        let string = std::fs::read_to_string(&file_path)
        .or(Err(DataStorageError::CantReadFromFile))?;

        match string.as_str() {
            "" => Ok(Self::default()),
            _  => serde_json::from_str::<ServerList>(&string).or(Err(DataStorageError::BadDataInFile)),
        }
    }

    pub fn store(&self, file_path: PathBuf) -> Result<(), DataStorageError> {
        // TODO: serde_json::to_writer takes a Write-able; try using it?
        let json_string = serde_json::to_string(self).or(Err(DataStorageError::CantSerializeSelf))?;

        write(file_path, json_string).or(Err(DataStorageError::CantWriteToFile))?;
        Ok(())
    }

    pub fn add_server(&mut self, url: Url, switch: bool) -> Result<&mut Server, Error> {

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
        if self.servers.len() == 1 { self.selected = Some(0); };
        if switch {self.set_server(self.servers.len() - 1)?; };

        return Ok(self.selected().expect("By now we should have a selected server"))
    }

    pub fn remove_server(&mut self, url: Url) -> Result<(), Error> {
        
        if self.servers.iter().all(|x| {x.url() != url}) {
            Err(RemoveServerError::ServerNotInList)?
        }
    
        self.servers.retain(|x| {x.url() != url});

        if self.servers.len() == 0 { self.selected = None } else
        if self.selected.expect("The selected element should be Some, since the length of the server list is non-zero") 
        > self.servers.len() - 1 { self.selected = Some(self.servers.len() - 1) }

        Ok(())
    }

    // NOTE: maybe return the selected server?
    pub fn set_server(&mut self, idx: usize) -> Result<&mut Server, Error> {
        if self.servers.is_empty() {
            Err(SelectedServerError::NoServers)?
        } else if idx >= self.servers.len() {
            Err(SelectedServerError::OutOfBounds{ idx, max: self.servers.len() - 1})?
        } else {
            self.selected = Some(idx);
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