mod args;

use super::gpg;
pub use args::Args;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn main(args: Args) {
    let path = super::args::root().join(args.item);
    let content = gpg::decrypt(&path).expect("Failed to decrypt");
    let edited_content = editor(content).expect("Failed to edit");
    gpg::encrypt(path, edited_content).expect("Failed to encrypt");
}

fn editor(content: String) -> Result<String, Box<dyn Error>> {
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(content.as_bytes())?;
    Command::new(get_editor()).arg(temp_file.path()).status()?;
    let mut edited_file = File::open(temp_file.path())?;
    let mut edited_output = String::new();
    edited_file.read_to_string(&mut edited_output)?;
    Ok(edited_output)
}

fn get_editor() -> String {
    env::var("EDITOR").unwrap_or("vim".to_string())
}
