use reqwest::{blocking::Response};
use anyhow::Error;
use url::Url;

use crate::posts::Post;

pub struct Server {   
    url: Url, 
}

impl Server {
    pub fn get_post(&self, id: u16) -> Result<Post, Error> {
        // https://docs.rs/url/latest/url/struct.Url.html#method.parse_with_params
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
    fn publish(post: Post) -> Result<Response, Error> {
        // I think we need a Client 'cause we need headers and bodies and stuff to POST
        // https://docs.rs/reqwest/latest/reqwest/blocking/index.html#making-post-requests-or-setting-request-bodies
        let client = reqwest::blocking::Client::new();

        // https://docs.rs/reqwest/latest/reqwest/blocking/struct.RequestBuilder.html
        Ok(
            client.post("http://localhost:3000/posts")
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
}