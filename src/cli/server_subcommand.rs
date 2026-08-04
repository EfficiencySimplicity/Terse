use super::*;
use text_io::read;

use crate::network::server::LoginOption;

#[derive(Subcommand)]
pub enum ServerSubcommand {
    Add {url: Url},
    Remove {url: Url},
    Set {idx: usize},
    Login {email: String, password: String},
    // https://stackoverflow.com/questions/60458705/how-do-i-specify-a-boolean-command-line-flag-using-clap
    List {#[clap(short('p'), action)] show_passwords: bool},
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
                println!("I successfully set the server to {idx}: {}", server.identifier_string());
            }
            Self::List { show_passwords } => {
                println!("{}", if show_passwords {server_list.string_with_passwords()?} else {server_list.to_string()});
            }
            Self::Login { email, password } => {
                let server = server_list.selected()?;
                let account = Account::new(email, password);

                // THIS IS NOT TRUE
                // So we ask to sign in with this info;
                // And if the user is already verified and the password matches,
                // We're let in.
                // Otherwise the server creates the account and asks us for the code.

                // THIS IS CURRENTLY TRUE
                // We ask to 'create an account on the server'
                // I.e. "Server, trust that I own this email address!"
                // Well, the server don't trust us, it says, 'prove it!'
                // It sends us a code, and we use it as our account passcode,
                // doing a quick check first to the server to ask, 'hey, did we do it right?'
                // and so on.

                match server.request_login(&account)? {
                    LoginOption::PleaseVerify => {
                        println!("The server hasn't seen that email before, so you'll need to verify it.");
                        println!("It should have sent a code to your inbox; please enter it here:");
                        let code: String = read!("{}\n");

                        if !server.verify_user(&account, &code)? {
                            println!("The code was incorrect, please try again.");
                            return Ok(())
                        }
                    },
                    LoginOption::BadPassword => {
                        println!("The server said that you have the wrong password for {}", &account.email)
                    }
                    _ => {}
                }

                server.set_account(account.clone());
                println!("I successfully signed you into {} as {}", server.identifier_string(), &account.email);
            }
        }
        Ok(())
    }
}