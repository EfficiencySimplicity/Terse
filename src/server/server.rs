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

    pub(super) fn with_params<I, K, V>(&self, params: I) -> Result<Url, Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>
    {
        Ok(
            Url::parse_with_params(
                self.url.as_str(),
                params
            )?
        )
    }

    pub fn get_post(&self, id: u16) -> Result<Post, Error> {
        // https://docs.rs/url/latest/url/struct.Url.html#method.parse_with_params
        // TODO: change!
        Ok(
            reqwest::blocking::get(
                // Maybe not the most efficient, re-parsing, but it's negligible
                // (Because url::Urls ain't mut, or don't seem to be)
                Url::parse_with_params(
                    self.url.as_str(),
                    [("id", id.to_string())]
                )?
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
            client.post(self.url.clone())
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
            reqwest::blocking::get(self.url.clone())?
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
        write!(f, "Server running Terse version {}", self.version)?;
        write!(f, "Users on server: {}", self.users)?;
        write!(f, "Answers on server: {}", self.posts)?;
        write!(f, "Server instance age: {}", self.age)?;
        write!(f, "Maximum post size: {}", ByteSize::b(self.max_post_size))
    }
}