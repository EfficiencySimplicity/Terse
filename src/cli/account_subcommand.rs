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

                let mut email: String;

                // This needs to be D.R.Y. Do we even need to check emails?...
                loop {
                    println!("Email: ");
                    email = read!("{}\n");

                    match server.email_exists(&email) {
                        Ok(exists) => {
                            if exists {
                                println!("An account with the email address {email} already exists on {}", server.url())
                            } else {
                                println!("");
                                break
                            }
                        }
                        Err(e) => {
                            println!("I got an error when trying to check if an account with that email exists on {}:", server.url());
                            println!("{e}")
                        }
                    }
                    println!("");
                }

                let mut username: String;

                loop {
                    println!("Username (leave blank to use email): ");
                    username = read!("{}\n");

                    if username.is_empty() {
                        username = email.clone();
                        break;
                    }

                    // TODO: make this account_is_ok_to_create instead.
                    // send over an account, username, email, password defaults;
                    // server can send over whatever message it wants.
                    // And would that work? Could you tell it to ignore some fields;
                    // i.e. check email and ignore nonexistent username so far?
                    match server.username_exists(&username) {
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