use super::*;
use crate::network::AccountCreationMessage;

use text_io::read;

#[derive(Subcommand)]
pub enum AccountSubcommand {
    New,//(NewOptions, which is enum for the e, Opt<u>, pass OR empty for lil tui)
    Login {username: String, password: String},// Don't worry; the above comment makes no sense to me either
    Delete {username: String},// TODO: account identifiers? aliases? Overcomplicating, am I?
}// Oh yeah that comment makes sense now! Either pass in the username, password, etc, or leave it blank
// and it prompts you to enter things!

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

                // TODO: have a few types of errors that the server can json back;
                // or it it sorta unknown what the server'll do?
                // Like, it could auto-sign ya up, or tell you to go to a link...
                let return_message = server.create_account(Account::new(Some(email), username, password))?;

                println!("I asked the server to create your account, and it said:");
                println!("{}", return_message);

                // server request new account...
            }

            Self::Login { username, password } => {
                let account = Account::new(None, username.clone(), password);
                match 
                    ServerList::from_config_file()?
                    .op_and_store(
                        |selected| -> Result<(), Error> 
                        {selected.login_account(account)}
                    ) {
                    // TODO: server name
                    Ok(server) => println!("I successfully logged you into {} on ", username),
                    Err(e) => eprint!("{e}"),
                }
            }
            Self::Delete { username, } => {
                let mut server_list = ServerList::from_config_file()?;
                let mut server = server_list.selected()?;
                match server.request_delete_account(&username) {
                    Ok(_) => {
                        println!("I asked the server to send a code to your inbox; please enter it here:");
                        let code: String = read!("{}\n");
                        match server.finalize_delete_account(&username, code) {
                            Ok(_) => {
                                println!("The server successfully deleted your account!");
                                server_list.store()?
                            },
                            Err(e) => eprintln!("The server had a problem deleting your account: {e}")
                        }
                    },
                    Err(e) => eprint!("The server had a problem: {e}"),
                }
            }
        }
        Ok(())
    }
}