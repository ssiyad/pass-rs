use std::fs;

pub fn init() {
    let root = crate::args::root();
    let file_name = ".plain-id";
    let path = root.join(file_name);
    fs::write(path, "").unwrap();
}

pub fn decrypt(content: Vec<u8>) -> String {
    String::from_utf8(content).unwrap()
}

pub fn encrypt(content: String) -> Vec<u8> {
    content.as_bytes().to_vec()
}
