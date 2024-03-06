use inquire::{Select, Text};
use std::fs;
use std::path::PathBuf;
use std::{error::Error, path::Path};

pub fn run<T>(root: T) -> Result<(), Box<dyn Error>>
where
    T: AsRef<Path>,
{
    // Prompt user for category, domain, login, and password.
    let category = Select::new("Category", vec!["Websites", "Misc"])
        .prompt()?
        .to_lowercase();

    // Create path based on category.
    let mut path = root.as_ref().join(&category);
    match category.as_str() {
        "websites" => path = path_website(path)?,
        "misc" => path = path_misc(path)?,
        _ => unreachable!(),
    }

    let password = Text::new("Password").prompt()?;

    // Create missing directories. Skip first element, which will be the
    // file itself.
    for p in path.ancestors().skip(1).filter(|x| !x.exists()) {
        fs::create_dir(p).unwrap();
    }

    // Encrypt password and write to file.
    gpg::encrypt(path, password)
}

fn path_website(path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let domain = Text::new("Domain").prompt()?.to_lowercase();
    let login = Text::new("Login").prompt()?;
    Ok(path.join(domain).join(login))
}

fn path_misc(path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let name = Text::new("Name").prompt()?.to_lowercase();
    Ok(path.join(name))
}