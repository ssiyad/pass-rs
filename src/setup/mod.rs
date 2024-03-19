use super::gpg;
use inquire::Select;
use std::fs;

pub fn main() {
    let root = super::args::root();
    let keys = gpg::get_keys().expect("Failed to get keys");
    let path_key = root.join(".gpg-id");

    println!("Intializing in {}", root.display());

    // Create root directory if it doesn't exist.
    if !root.exists() {
        fs::create_dir(&root).expect("Failed to create root directory");
    }

    // Create key info file if it doesn't exist.
    if !path_key.exists() {
        let key;

        // If no keys are present, ask to create one.
        if keys.is_empty() {
            panic!("No keys found. Please create one.");
        }
        // If only one key is present, use it.
        else if keys.len() == 1 {
            key = keys.clone().pop().unwrap();
        }
        // If more than one key is present, ask for one.
        else {
            key = Select::new("Key", keys).prompt().unwrap();
        }

        // Write the key to the file.
        fs::write(path_key, key).expect("Failed to write key to file");
    }
}
