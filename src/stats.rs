use serde::{ Serialize, Deserialize };
use reqwest::{Error};
use bytesize::ByteSize;

#[derive(Deserialize)]
pub struct ServerStats {
    version: String,
    users: u32,
    posts: u32,
    age: f64,
    max_post_size: u64,
}


fn get_stats() -> Result<ServerStats, Error> {
    reqwest::blocking::get("http://localhost:3000/stats")?
    .json::<ServerStats>()
}

pub fn display_stats() {
    let res = get_stats();
    
    match res {
        Err(e) => {println!(
            // TODO: exact message / version command hint...
            // TODO: autoupdater?
            // TODO: How will you get the server's version??
            "Error in getting server stats; The issue may be:
            -    Server is down or not accessible
            -    Server is using a different stats format;
                 Check the first 2 numbers of your Terse version match the server's
            
            Error message: {}", e)}
        Ok(stats) => {
            // TODO: that bookmarked clean-list library
            // TODO: coloration
            println!("Server running Terse version {}", stats.version);
            println!("Users on server: {}", stats.users);
            println!("Answers on server: {}", stats.posts);
            println!("Server instance age: {}", stats.age);
            println!("Maximum post size: {}", ByteSize::b(stats.max_post_size));
        }
    }
}