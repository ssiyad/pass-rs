use std::{error::Error, path::PathBuf};

pub fn run(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let content = gpg::decrypt(path)?;
    println!("{}", content);
    Ok(())
}
