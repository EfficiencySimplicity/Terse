use crate::tui::Selectable;
use crate::network::{Account, Server};

use directories::ProjectDirs;
use std::fs::*;

use url::Url;

use serde::Deserialize;

use anyhow::Error;

//pub type ServerList = Selectable<Server>;

// TODO: make Server accept Client, so it can be Deserialize, so there's no need for builders.
#[derive(Deserialize)]
pub struct ServerListBuilder {
    pub servers: Vec<ServerBuilder>
}

#[derive(Deserialize)]
pub struct ServerBuilder {
    pub url: String,
    pub accounts: Vec<Account>,
}

impl ServerListBuilder {
    fn build(self) -> Result<ServerList, Error> {
        // https://stackoverflow.com/questions/63798662/how-do-i-convert-a-vecresultt-e-to-resultvect-e
        Ok(ServerList { servers: self.servers.into_iter().map(ServerBuilder::build).collect::<Result<Vec<Server>, Error>>()? })
    }
}

impl ServerBuilder {
    fn build(self) -> Result<Server, Error> {
        Ok(Server::with_accounts(Url::parse(&self.url)?, self.accounts))
    }
}

pub struct ServerList {
    pub servers: Vec<Server>,
}

impl ServerList {
    pub fn from_config_file() -> Result<ServerList, Error> {
        // https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
        let dirs = ProjectDirs::from("", "InsanityOnAMachine", "Terse").ok_or(Error::msg("Home directory not found"))?;
        let data_dir = dirs.data_dir();
        let servers_file = data_dir.join("servers");

        // NOTE: there are notes saying you should use try_exists() sometimes.
        match &mut File::open(&servers_file) {
            Ok(f) => {
                // NOTE: This could be a std::io::BufReader that wraps f;
                // Would that be any better?
                let server_list_builder: ServerListBuilder = serde_json::from_reader(f)?;
                return server_list_builder.build()
            }
            Err(e) => {
                File::create(&servers_file)?;
                return Self::from_config_file();
            }
        }
    }
}

// Literally window and widget can both be implemented already...