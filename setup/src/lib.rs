use inquire::Select;
use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = config::parse();
    let root = config.root();
    let keys = gpg::get_keys()?;
    let path_key = root.join(".gpg-id");

    println!("Intializing in {}", root.display());

    // Create root directory if it doesn't exist.
    if !root.exists() {
        fs::create_dir(&root)?;
    }

    // Create key info file if it doesn't exist.
    if !path_key.exists() {
        let key;

        // If no keys are present, ask to create one.
        if keys.len() == 0 {
            return Err("No keys found. Please create one.".into());
        }
        // If only one key is present, use it.
        else if keys.len() == 1 {
            key = keys.clone().pop().unwrap();
        }
        // If more than one key is present, ask for one.
        else {
            key = Select::new("Key", keys).prompt()?;
        }

        // Write the key to the file.
        fs::write(path_key, key)?;
    }

    Ok(())
}
