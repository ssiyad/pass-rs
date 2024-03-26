use std::fs;

/// For plain, there is nothing extra to do.
pub fn init() {
    let root = crate::args::root();
    let crypto_config = root.join(".pass").join("crypto");
    fs::write(crypto_config, "plain").unwrap();
}

pub fn decrypt(content: Vec<u8>) -> String {
    String::from_utf8(content).unwrap()
}

pub fn encrypt(content: String) -> Vec<u8> {
    content.as_bytes().to_vec()
}
