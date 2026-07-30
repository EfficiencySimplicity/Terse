use super::*;

#[derive(Subcommand)]
pub enum DevSubcommand {
    #[command(about = "Deletes an account via the username and code")]
    DeleteAccount {username: String, code: String},
}

impl DevSubcommand {
    pub fn process(self) -> Result<(), Error> {
        match self {

            DevSubcommand::DeleteAccount {username, code} => {
                let mut server_list = ServerList::from_config_file()?;
                let mut server = server_list.selected()?;
                
                match server.finalize_delete_account(&username, code) {
                    Ok(_) => {
                        println!("The server successfully deleted your account!");
                        server_list.store()?
                    },
                    Err(e) => eprintln!("The server had a problem deleting your account: {e}")
                }
            }

        }
        Ok(())
    }
}