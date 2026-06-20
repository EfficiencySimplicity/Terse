use serde::{ Serialize, Deserialize };
use bytesize::ByteSize;
use std::fmt::Formatter;
use std::fmt::Display;
use indoc::writedoc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub content: String,
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