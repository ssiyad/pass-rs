/// For plain, there is nothing extra to do.
pub fn init() {}

pub fn decrypt(content: Vec<u8>) -> String {
    String::from_utf8(content).unwrap()
}

pub fn encrypt(content: String) -> Vec<u8> {
    content.as_bytes().to_vec()
}
