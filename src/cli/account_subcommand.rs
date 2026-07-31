use super::*;
use crate::network::AccountCreationMessage;

use text_io::read;

#[derive(Subcommand)]
pub enum AccountSubcommand {
    New,//(NewOptions, which is enum for the e, Opt<u>, pass OR empty for lil tui)
    Login {username: String, password: String},// Don't worry; the above comment makes no sense to me either
    Delete {username: String},// TODO: account identifiers? aliases? Overcomplicating, am I?
    List,
}// Oh yeah that comment makes sense now! Either pass in the username, password, etc, or leave it blank
// and it prompts you to enter things!

impl AccountSubcommand {
    pub fn process(self, server_list: &mut ServerList) -> Result<(), Error> {
        match self {
            Self::New => {
                let server = server_list.selected()?;

                println!("Creating new account on server {}:\n", server.url());

                let mut email: String;

                // This needs to be D.R.Y. Do we even need to check emails?...
                loop {
                    println!("Email: ");
                    email = read!("{}\n");

                    match server.could_create_account(Some(&email), None) {
                        Ok(message) => {
                            match message {
                                AccountCreationMessage::Sure => {
                                    println!("");
                                    break;
                                }
                                AccountCreationMessage::Nope(reason) => {
                                    println!("{reason}")
                                }
                            }
                        }
                        Err(e) => {
                            println!("I got an error when trying to check if there could be any problem creating an account with that email on {}:", server.url());
                            println!("{e}")
                        }
                    }
                    println!("");
                }

                let mut username: String;

                loop {
                    println!("Username (leave blank to use email): ");
                    username = read!("{}\n");

                    // Don't bother checking; the email is fine and nobody can make a
                    // username that's a valid email
                    if username.is_empty() {
                        username = email.clone();
                        break;
                    }

                    // TODO: make this account_is_ok_to_create instead.
                    // send over an account, username, email, password defaults;
                    // server can send over whatever message it wants.
                    // And would that work? Could you tell it to ignore some fields;
                    // i.e. check email and ignore nonexistent username so far?
                    match server.could_create_account(None, Some(&username)) {
                        Ok(message) => {
                            match message {
                                AccountCreationMessage::Sure => {
                                    println!("");
                                    break;
                                }
                                AccountCreationMessage::Nope(reason) => {
                                    println!("{reason}")
                                }
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

                let account = Account::new(Some(email), username.clone(), password);

                // This is a sweet amount of code duplication. Really! REALLY!
                match server.request_create_account(account.clone()) {
                    Ok(_) => {
                        println!("I asked the server to send a code to your inbox; please enter it here:");
                        let code: String = read!("{}\n");
                        match server.finalize_create_account(account, &code) {
                            Ok(_) => {
                                println!("The server successfully created your account!");
                                server_list.store()?
                            },
                            Err(e) => eprintln!("The server had a problem creating your account: {e}")
                        }
                    },
                    Err(e) => eprint!("The server had a problem: {e}"),
                }
            }

            Self::Login { username, password } => {
                let account = Account::new(None, username.clone(), password);
                let server = server_list.selected()?;
                match server.login_account(account) {
                    // TODO: have a server command that gets a name field; "url" (name)
                    Ok(_) => println!("I successfully logged you into {} on {}", username, server.url()),
                    Err(e) => eprint!("{e}"),
                }
            }
            Self::Delete { username, } => {
                let server = server_list.selected()?;
                match server.request_delete_account(&username) {
                    Ok(_) => {
                        println!("I asked the server to send a code to your inbox; please enter it here:");
                        let code: String = read!("{}\n");
                        match server.finalize_delete_account(&username, code) {
                            Ok(_) => {
                                println!("The server successfully deleted your account!");
                            },
                            Err(e) => eprintln!("The server had a problem deleting your account: {e}")
                        }
                    },
                    Err(e) => eprint!("The server had a problem: {e}"),
                }
            },
            Self::List => Self::process_list(server_list)?
        }
        Ok(())
    }

    // TODO: type alias for Result<(), Error>?
    fn process_list(server_list: &ServerList) -> Result<(), Error> {
        println!("{}", server_list.clone_selected()?);
        Ok(())
    }
}