use reqwest::{Error};
use url::Url;

use crate::posts::Post;

pub struct Server {   
    url: Url, 
}

impl Server {
    pub fn get_post(&self, id: u16) -> Result<Post, Error> {
        // https://docs.rs/url/latest/url/struct.Url.html#method.parse_with_params
        let post = reqwest::blocking::get(
            // Maybe not the most efficient, re-parsing, but it's negligible
            // (Because url::Urls ain't mut, or don't seem to be)
            Url::parse_with_params(
                self.url.as_str(),
                [("id", id)]
            )
        );?
        .json::<Post>()?;

        return Ok(post);
    }
}