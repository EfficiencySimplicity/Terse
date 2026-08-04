use serde::{ Serialize, Deserialize };
use bytesize::ByteSize;
use std::fmt::Formatter;
use std::fmt::Display;
use indoc::writedoc;

// This could all be in network, in posts or publishing, etc...

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub content: String,
}

#[derive(thiserror::Error, Deserialize, Debug)]
pub struct PostSizeError {
    max_post_size: u64,
    // max_title_length
}

impl Display for PostSizeError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        // https://stackoverflow.com/questions/33334994/multi-line-string-in-rust-with-preserved-leading-whitespace
        writedoc!(
            f,
            "
            The server rejected your post because part of it was too large;
            Make sure:
                - The content is within {}
            ",
            ByteSize::b(self.max_post_size)
        )
    }
}

// Assuming you are logged in, and you get a valid post to send,
// This represents all errors the server can send back
// that you'd have no way of checking client-side
pub enum PublishingError {
    PostTooLarge { max_size: u64 },
    Timeout { time_remaining: u64 },
}