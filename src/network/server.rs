use reqwest::{blocking::Response};
use anyhow::Error;
use url::Url;
use std::fmt::{Display, Formatter};
use bytesize::ByteSize;

use std::borrow::Borrow;

use serde::{Deserialize};

use crate::posts::Post;

pub struct Server {   
    url: Url, 
}

impl Server {

    pub fn new(url: Url) -> Self {
        Self {url}
    }

    // I opt to use &strs instead of Options as the arguments, although setting query = None is really fun...
    // But it'd be a lotta extra text that could just be an empty &str
    pub(super) fn with_params(&self, route: &str, query: &str)
    {
        let url = self.url.clone();
        url.set_path(route);
        url.set_query(query);
        return url;
    }

    pub fn get_post(&self, id: u16) -> Result<Post, Error> {
        // https://docs.rs/url/latest/url/struct.Url.html#method.parse_with_params
        Ok(
            reqwest::blocking::get(
                self.with_params("posts", format!("id={}", id.to_string()))
            )?
            .json::<Post>()?
        )
    }

    // NOTE: In the end, this should be async so it don't block the TUI
    pub fn publish(&self, post: Post) -> Result<Response, Error> {
        // I think we need a Client 'cause we need headers and bodies and stuff to POST
        // https://docs.rs/reqwest/latest/reqwest/blocking/index.html#making-post-requests-or-setting-request-bodies
        let client = reqwest::blocking::Client::new();

        // https://docs.rs/reqwest/latest/reqwest/blocking/struct.RequestBuilder.html
        Ok(
            client.post(self.with_params("posts", ""))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&post)?)
            .send()?
        )

        // match result {
        //     // https://docs.rs/reqwest/latest/reqwest/struct.StatusCode.html
        //     Ok(response) => match response.status().as_u16() {
        //         413 => {
        //             let err = response.json::<PostSizeExceptionError>()?;
        //             println!("{err}");
        //         }
        //         _ => println!("Answer published successfully"),
        //     }
        //     Err(err) => println!("Error in sending post: {err}")
        // }
        // Ok(())
    }

    pub fn get_stats(&self) -> Result<ServerStats, Error> {
        Ok(
            // TODO: separate methods for getting self.url with path vs. with params vs. both?...
            reqwest::blocking::get(self.with_params("stats", ""))?
            .json::<ServerStats>()?
        )
    }
}

#[derive(Deserialize)]
pub struct ServerStats {
    version: String,
    users: u32,
    posts: u32,
    age: f64,
    max_post_size: u64,
}

impl Display for ServerStats {

    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // TODO: that bookmarked clean-list library
        // TODO: coloration
        writeln!(f, "Server running Terse version {}", self.version)?;
        writeln!(f, "Users on server: {}", self.users)?;
        writeln!(f, "Answers on server: {}", self.posts)?;
        writeln!(f, "Server instance age: {}", self.age)?;
        writeln!(f, "Maximum post size: {}", ByteSize::b(self.max_post_size))
    }
}