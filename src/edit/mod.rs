mod args;

use crate::crypto;
use crate::storage;
pub use args::Args;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn main(args: Args) {
    let crypto_backend = crypto::get();
    let storage_backend = storage::get();
    let content_raw = storage_backend.read(args.item.clone());
    let content = crypto_backend.decrypt(content_raw);
    let edited_content = editor(content).expect("Failed to edit");
    let content_encrypted = crypto_backend.encrypt(edited_content);
    storage_backend.write(args.item, content_encrypted);
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
