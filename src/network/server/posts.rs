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
            The server rejected your post because part of it was too large;
            Make sure:
                - The content is within {}
            ",
            ByteSize::b(self.max_post_size)
        )
    }
}