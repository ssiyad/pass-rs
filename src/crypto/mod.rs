mod gpg;
mod plain;

use clap::ValueEnum;
use std::fs;

#[derive(Clone, ValueEnum)]
pub enum Backend {
    Gpg,
    Plain,
}

impl Backend {
    /// Initialization steps for the crypto backend
    pub fn init(&self) {
        match self {
            Backend::Gpg => gpg::init(),
            Backend::Plain => plain::init(),
        }
    }

    /// Decrypt the content
    ///
    /// * `content`:
    pub fn decrypt(&self, content: Vec<u8>) -> String {
        match self {
            Backend::Gpg => gpg::decrypt(content),
            Backend::Plain => plain::decrypt(content),
        }
    }

    /// Encrypt the content
    ///
    /// * `content`:
    pub fn encrypt(&self, content: String) -> Vec<u8> {
        match self {
            Backend::Gpg => gpg::encrypt(content),
            Backend::Plain => plain::encrypt(content),
        }
    }
}

/// Get the crypto backend
pub fn get() -> Backend {
    let root = crate::args::root();
    let crypto_config = root.join(".pass").join("crypto");
    let crypto_type = fs::read_to_string(crypto_config).unwrap().trim().to_owned();

    match crypto_type.as_str() {
        "gpg" => Backend::Gpg,
        "plain" => Backend::Plain,
        _ => panic!("No crypto setup"),
    }
}
