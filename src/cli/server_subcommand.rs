use super::*;

#[derive(Subcommand)]
pub enum ServerSubcommand {
    Add {url: Url},
    Remove {url: Url},
    Set {idx: usize},
    List,
}

impl ServerSubcommand {
    pub fn process(self, server_list: &mut ServerList) -> Result<(), Error> {
        match self {
            Self::Add { url } => {
                server_list.add_server(url.clone())?;
                println!("I successfully added {url} to the list of servers!");
            }
            Self::Remove { url } => {
                server_list.remove_server(url.clone())?;
                println!("I successfully removed {} from the list of servers", url);
            }
            Self::Set { idx } => {
                let server = server_list.set_server(idx)?;
                // If this wanted more stats... I'd just not globally error;
                // I'd say "I successfully set the server to 4: [couldn't get name]""
                println!("I successfully set the server to {idx}: {}", server.url());
            }
            Self::List => {
                println!("{server_list}");
            }
        }
        Ok(())
    }
}