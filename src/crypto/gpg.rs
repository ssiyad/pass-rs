use gpgme::{Context, Protocol};
use inquire::Select;
use std::fs;

/// Get the GPG context.
fn get_context() -> Context {
    Context::from_protocol(Protocol::OpenPgp).unwrap()
}

/// Get secret keys.
fn get_keys() -> Vec<String> {
    let mut context = get_context();
    let keys = context.secret_keys().unwrap();
    keys.filter_map(|x| x.ok())
        .map(|x| x.fingerprint().unwrap().to_owned())
        .collect()
}

/// Get key used for recipients.
fn get_key() -> String {
    let root = crate::args::root();
    let path_key = root.join(".pass").join("gpg-id");
    fs::read_to_string(path_key)
        .expect("Failed to read key from file")
        .trim()
        .to_owned()
}

pub fn init() {
    let root = crate::args::root();
    let path_key = root.join(".pass").join("gpg-id");
    let keys = get_keys();

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

pub fn decrypt(content: Vec<u8>) -> String {
    let mut output = Vec::new();
    get_context().decrypt(content, &mut output).unwrap();
    String::from_utf8(output).unwrap()
}

pub fn encrypt(content: String) -> Vec<u8> {
    let key_str = get_key();
    let mut context = get_context();
    let mut output = Vec::new();
    let key = context.get_key(key_str).unwrap();
    let recp = vec![&key];
    context
        .encrypt(recp, content.as_bytes(), &mut output)
        .unwrap();
    output
}
