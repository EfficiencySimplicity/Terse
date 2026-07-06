use super::*;

use text_io::read;

#[derive(Subcommand)]
pub enum AccountSubcommand {
    New,
}

impl AccountSubcommand {
    pub fn process(self) -> Result<(), Error> {
        match self {
            Self::New => {
                let mut server_list = ServerList::from_config_file()?;
                let server = server_list.selected()?;

                println!("Creating new account on server {}:\n", server.url());

                println!("Email: ");
                let email: String = read!("{}\n");
                println!("");

                let mut username: String;

                loop {
                    println!("Username: ");
                    username = read!("{}\n");

                    match server.account_exists(&username) {
                        Ok(exists) => {
                            if exists {
                                println!("An account by the name of {username} already exists on {}", server.url())
                            } else {
                                println!("");
                                break
                            }
                        }
                        Err(e) => {
                            println!("I got an error when trying to check if an account with that username exists on {}:", server.url());
                            println!("{e}")
                        }
                    }
                    println!("");
                }

                println!("Password: ");
                let password: String = read!("{}\n");
                println!("");

                // TODO: have a few types of errors that the server can json back;
                // or it it sorta unknown what the server'll do?
                // Like, it could auto-sign ya up, or tell you to go to a link...
                let return_message = server.create_account(Account::new(email, username, password))?;

                println!("I asked the server to create your account, and it said:");
                println!("{}", return_message);

                // server request new account...
            }
        }
        Ok(())
    }
}