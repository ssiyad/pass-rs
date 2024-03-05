use gpgme::{Context, Protocol};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn run() -> Result<(), Box<dyn Error>> {
    let path = "/tmp/ssiyad.gpg";
    let content = decrypt(path)?;
    let edited_content = editor(content)?;
    encrypt(path, edited_content)?;
    Ok(())
}

fn decrypt(path: &str) -> Result<String, Box<dyn Error>> {
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;
    let mut input = File::open(path)?;
    let mut output = Vec::new();
    ctx.decrypt(&mut input, &mut output)?;
    Ok(String::from_utf8(output)?)
}

fn editor(content: String) -> Result<String, Box<dyn Error>> {
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(content.as_bytes())?;
    Command::new("nvim").arg(temp_file.path()).status()?;
    let mut edited_file = File::open(temp_file.path())?;
    let mut edited_output = String::new();
    edited_file.read_to_string(&mut edited_output)?;
    Ok(edited_output)
}

fn encrypt(path: &str, content: String) -> Result<(), Box<dyn Error>> {
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;
    let mut output = Vec::new();
    ctx.encrypt(Vec::new(), content, &mut output)?;
    let mut output_file = File::create(path)?;
    output_file.write_all(&output)?;
    Ok(())
}
