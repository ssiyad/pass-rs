use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = config::parse();
    let root = config.root();
    let cmd = config.command();

    if let config::Command::Show { item } = cmd {
        let path = root.join(item);
        let content = gpg::decrypt(path)?;
        println!("{}", content);
    }

    Ok(())
}
