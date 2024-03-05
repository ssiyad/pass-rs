use gpgme::{Context, Protocol};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn decrypt(path: &str) -> Result<String, Box<dyn Error>> {
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;
    let mut input = File::open(path)?;
    let mut output = Vec::new();
    ctx.decrypt(&mut input, &mut output)?;
    Ok(String::from_utf8(output)?)
}

pub fn encrypt(path: &str, content: String) -> Result<(), Box<dyn Error>> {
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;
    let mut output = Vec::new();
    ctx.encrypt(Vec::new(), content, &mut output)?;
    let mut output_file = File::create(path)?;
    output_file.write_all(&output)?;
    Ok(())
}
