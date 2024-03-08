use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = config::parse();
    let root = config.root();

    if let config::Command::Edit { item } = config.command() {
        let path = root.join(item);
        let content = gpg::decrypt(&path)?;
        let edited_content = editor(content)?;
        gpg::encrypt(path, edited_content)?;
    }

    Ok(())
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
