use gpgme::{Context, Protocol};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Decrypt a file and return the content.
pub fn decrypt<T>(path: T) -> Result<String, Box<dyn Error>>
where
    T: AsRef<Path>,
{
    let mut input = File::open(path.as_ref().with_extension("gpg"))?;
    let mut output = Vec::new();
    get_context()?.decrypt(&mut input, &mut output)?;
    Ok(String::from_utf8(output)?)
}

/// Encrypt a file with the given content.
pub fn encrypt<T>(path: T, content: String) -> Result<(), Box<dyn Error>>
where
    T: AsRef<Path>,
{
    let mut output = Vec::new();
    let mut output_file = File::create(path.as_ref().with_extension("gpg"))?;
    get_context()?.encrypt(Vec::new(), content, &mut output)?;
    Ok(output_file.write_all(&output)?)
}

/// Get a list of all secret keys.
pub fn get_keys() -> Result<Vec<String>, Box<dyn Error>> {
    let keys = get_context()?
        .secret_keys()?
        .filter_map(|key| key.ok())
        .filter_map(|key| key.id().map(|id| id.to_string()).ok())
        .collect::<Vec<String>>();
    Ok(keys)
}

fn get_context() -> Result<Context, gpgme::Error> {
    Context::from_protocol(Protocol::OpenPgp)
}
