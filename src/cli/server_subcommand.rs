use super::*;

#[derive(Subcommand)]
pub enum ServerSubcommand {
    Add {url: Url},
    Remove {url: Url},
    Set {idx: usize},
    List,
}

impl ServerSubcommand {
    pub fn process(self) -> Result<(), Error> {
        match self {
            Self::Add { url } => {
                ServerList::from_config_file()?.add_server(url.clone())?;
                println!("I successfully added {url} to the list of servers!");
            }
            Self::Remove { url } => {
                ServerList::from_config_file()?.remove_server(url.clone())?;
                println!("I successfully removed {} from the list of servers", url);
            }
            Self::Set { idx } => {
                let mut server_list = ServerList::from_config_file()?;
                let server = server_list.set_server(idx)?;
                // If this wanted more stats... I'd just not globally error;
                // I'd say "I successfully set the server to 4: [couldn't get name]""
                println!("I successfully set the server to {idx}: {}", server.url());
            }
            Self::List => {
                println!("{}", ServerList::from_config_file()?);
            }
        }
        Ok(())
    }
}