use crate::cli::*;
use serde::Deserialize;
use reqwest::{Error, blocking::get};

// TODO: use the directories crate to set the server.
// and a localhost flag -l for easy no-type-full-url

pub fn run_commands(command: CommandsCli) {
    match command.command {
        Commands::Stats => display_stats(),
        // TODO: This should not even be a command, maybe later
        Commands::GetPost => display_post(),
        _ => {}
    }
}

#[derive(Deserialize)]
pub struct ServerStats {
    version: String,
    users: u32,
    answers: u32,
    age: f64,
}


fn get_stats() -> Result<ServerStats, Error> {
    let stats = reqwest::blocking::get("http://localhost:3000/stats")?
    .json::<ServerStats>()?;

    return Ok(stats);    
}

fn display_stats() {
    let res = get_stats();
    
    match res {
        Err(e) => {println!(
            // TODO: exact message / version command hint...
            // TODO: autoupdater?
            "Error in getting server stats; The issue may be:
            -    Server is down or not accessable
            -    Server is using a different stats format; check your version;
            
            Error message: {}", e)}
        Ok(stats) => {
            // TODO: that bookmarked clean-list library
            // TODO: coloration
            println!("Server running Terse version {}", stats.version);
            println!("Users on server: {}", stats.users);
            println!("Answers on server: {}", stats.answers);
            println!("Server instance age: {}", stats.age);
        }
    }
}

#[derive(Deserialize)]
pub struct Post {
    title: String,
    content: String,
}

pub fn get_post(id: i32) -> Result<Post, Error> {
    let post = reqwest::blocking::get(format!("http://localhost:3000/posts?id={id}"))?
    .json::<Post>()?;

    return Ok(post);
}

fn display_post() {
    let res = get_post(0);
    
    match res {
        Err(e) => {println!(
            "Error in getting post;
            Error message: {}", e)}
        Ok(post) => {
            // TODO: that bookmarked clean-list library
            // TODO: coloration
            println!("{}", post.title);
            println!();
            println!("{}", post.content);
        }
    }
}