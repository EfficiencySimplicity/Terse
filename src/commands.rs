use crate::cli::*;
use serde::{ Serialize, Deserialize };
use reqwest::{Error};
use bytesize::ByteSize;
use std::fmt::Formatter;
use std::fmt::Display;
use indoc::writedoc;
use crate::stats::*;

// TODO: use the directories crate to set the server.
// and a localhost flag -l for easy no-type-full-url

// TODO: make indentation of size error correct

// 1 kb, and cli.rs goes ok, but commands.rs fails

pub fn run_commands(command: CommandsCli) {
    match command.command {
        Commands::Stats => display_stats(),
        // TODO: This should not even be a command, maybe later
        Commands::GetPost{ id } => display_post(id),
        Commands::Pub{ title, path } => try_publish(title, path),
        _ => {}
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

fn try_publish(title: String, path: String) {
    let content = std::fs::read_to_string(&path);

    match content {
        Ok(content) => {let _ = add_post(title, content);}
        Err(e) => println!("Could not read file at {path}")
    }
}

#[derive(Deserialize)]
pub struct PostSizeExceptionError {
    max_post_size: u64,
    // max_title_length
}

impl Display for PostSizeExceptionError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        // https://stackoverflow.com/questions/33334994/multi-line-string-in-rust-with-preserved-leading-whitespace
        writedoc!(
            f,
            "
            Part of your Answer was too large;
            Make sure:
                - The content is within {}
            ",
            ByteSize::b(self.max_post_size)
        )
    }
}

// TODO: all references!
// https://stackoverflow.com/questions/65814450/how-to-post-a-file-using-reqwest
fn add_post(title: String, content: String) -> Result<(), Box<dyn std::error::Error>> {
    // https://docs.rs/reqwest/latest/reqwest/blocking/index.html#making-post-requests-or-setting-request-bodies
    let client = reqwest::blocking::Client::new();

    // https://docs.rs/reqwest/latest/reqwest/blocking/struct.RequestBuilder.html
    let result = client.post("http://localhost:3000/posts")
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .body(serde_json::to_string(&Post {title, content})?)
    .send();

    match result {
        // https://docs.rs/reqwest/latest/reqwest/struct.StatusCode.html
        Ok(response) => match response.status().as_u16() {
            413 => {
                let err = response.json::<PostSizeExceptionError>()?;
                println!("{err}");
            }
            _ => println!("Answer published successfully"),
        }
        Err(err) => println!("Error in sending post: {err}")
    }
    Ok(())
}