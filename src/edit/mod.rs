mod args;

use crate::crypto;
pub use args::Args;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn main(args: Args) {
    let path = super::args::root().join(args.item);
    let content_raw = fs::read(&path).expect("Failed to read file");
    let content = crypto::get().decrypt(content_raw);
    let edited_content = editor(content).expect("Failed to edit");
    let content_encrypted = crypto::get().encrypt(edited_content);
    fs::write(path, content_encrypted).expect("Failed to write file");
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
