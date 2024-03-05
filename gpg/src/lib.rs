use gpgme::{Context, Protocol};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn decrypt<T>(path: T) -> Result<String, Box<dyn Error>>
where
    T: AsRef<Path>,
{
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;
    let mut input = File::open(path.as_ref().with_extension("gpg"))?;
    let mut output = Vec::new();
    ctx.decrypt(&mut input, &mut output)?;
    Ok(String::from_utf8(output)?)
}

pub fn encrypt<T>(path: T, content: String) -> Result<(), Box<dyn Error>>
where
    T: AsRef<Path>,
{
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;
    let mut output = Vec::new();
    ctx.encrypt(Vec::new(), content, &mut output)?;
    let mut output_file = File::create(path.as_ref().with_extension("gpg"))?;
    Ok(output_file.write_all(&output)?)
}
