use crate::network::ServerList;
use crate::posts::Post;

use anyhow::Error;

use std::{
    env::{temp_dir, var},
    fs::File,
    process::Command,
    path::PathBuf,
};

// https://docs.rs/capitalize/latest/capitalize/index.html
use capitalize::Capitalize;

pub fn process(server_list: &mut ServerList, title: Option<String>, path: Option<PathBuf>) -> Result<(), Error> {
    let server = server_list.selected()?;

    let title = match title {
        Some(s) => s,
        None => format_title(
            String::from(
                path.as_ref().expect("The path should be Some, since title is None")
                .file_name().unwrap()
                .to_str().unwrap()))
    };
    
    let content = match path {
        Some(path) => std::fs::read_to_string(&path)
        .or(Err(Error::msg("I couldn't read the path you gave me")))?,
        None => get_editor_input()?,
    };

    let post = Post {title: title.clone(), content: content};

    let message = server.publish(post)?;

    println!("{message}");
    Ok(())
}

fn format_title(title: String) -> String {
    return title.split(|x: char| x.is_whitespace() || x == '-' || x == '_')
    // I don't have to capitalize myself, thanks to this crate...
    // https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
    .map(|x: &str| x.capitalize())
    .collect::<Vec<String>>()
    .join(" ")
}

fn get_editor_input() -> Result<String, Error> {
    let editor = match var("EDITOR") {
        Ok(v) => v,
        Err(_) => String::from("vim")
    };
    
    let mut file_path = temp_dir();
    file_path.push("terse-post");
    if let Err(e) = File::create(&file_path) {
        return Err(Error::msg("I couldn't create a temporary file to store your post in"))
    }

    Command::new(editor)
        .arg(&file_path)
        .status()
        .expect("Something went wrong");

    return Ok(std::fs::read_to_string(&file_path)?)
}

// // Maybe make custom errors for publishing, etc etc etc...
//     fn process_pub(server_list: &mut ServerList, title: String, path: PathBuf) -> Result<(), Error> {
//         let server = server_list.selected()?;

//         // There could be a wrapper for fs errors that provides better printing
//         // 'I couldn't read the path' is good enough, and after a semicolon; great!
//         let content = std::fs::read_to_string(&path)
//             .or(Err(Error::msg("I couldn't read the path you gave me")))?;

//         // TODO: Is there a better way to manage content with the above match
//         // to avoid unwrapping?
//         let post = Post {title: title.clone(), content: content};

//         let message = server.publish(post)?;

//         println!("{message}");
//         Ok(())
//     }