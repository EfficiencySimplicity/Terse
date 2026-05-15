use crate::cli::*;
use serde::{ Serialize, Deserialize };
use reqwest::{Error};

// TODO: use the directories crate to set the server.
// and a localhost flag -l for easy no-type-full-url

pub fn run_commands(command: CommandsCli) {
    match command.command {
        Commands::Stats => display_stats(),
        // TODO: This should not even be a command, maybe later
        Commands::GetPost{ id } => display_post(id),
        Commands::AddPost{ title, content } => try_add_post(title, content),
        _ => {}
    }
}

#[derive(Deserialize)]
pub struct ServerStats {
    version: String,
    users: u32,
    posts: u32,
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
            -    Server is down or not accessible
            -    Server is using a different stats format; check your version;
            
            Error message: {}", e)}
        Ok(stats) => {
            // TODO: that bookmarked clean-list library
            // TODO: coloration
            println!("Server running Terse version {}", stats.version);
            println!("Users on server: {}", stats.users);
            println!("Answers on server: {}", stats.posts);
            println!("Server instance age: {}", stats.age);
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    title: String,
    content: String,
}

pub fn get_post(id: i32) -> Result<Post, Error> {
    let post = reqwest::blocking::get(format!("http://localhost:3000/posts?id={id}"))?
    .json::<Post>()?;

    return Ok(post);
}

fn display_post(id: i32) {
    let res = get_post(id);
    
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

fn try_add_post(title: String, content: String) {
    let x = add_post(title, content);
}

// TODO: all references!
// https://stackoverflow.com/questions/65814450/how-to-post-a-file-using-reqwest
fn add_post(title: String, content: String) -> Result<(), Box<dyn std::error::Error>> {
    // https://docs.rs/reqwest/latest/reqwest/blocking/index.html#making-post-requests-or-setting-request-bodies
    let client = reqwest::blocking::Client::new();

    println!("{}", serde_json::to_string(&Post {title: title.clone(), content: content.clone()})?);

    // https://docs.rs/reqwest/latest/reqwest/blocking/struct.RequestBuilder.html
    let result = client.post("http://localhost:3000/posts")
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .body(serde_json::to_string(&Post {title, content})?)
    .send()?;
    Ok(())
}