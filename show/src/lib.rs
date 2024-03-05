use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let path = "/tmp/ssiyad.gpg";
    let content = gpg::decrypt(path)?;
    println!("{}", content);
    Ok(())
}

