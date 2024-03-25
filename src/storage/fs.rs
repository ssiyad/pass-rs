use std::{fs, path::PathBuf};

pub fn init() {
    let root = crate::args::root();
    let crypto_config = root.join(".pass").join("storage");
    fs::write(crypto_config, "fs").unwrap();
}

fn path(name: String) -> PathBuf {
    crate::args::root().join(name)
}

pub fn read(name: String) -> Vec<u8> {
    fs::read(path(name)).unwrap()
}

pub fn write(name: String, content: Vec<u8>) {
    let effective_path = path(name);
    fs::create_dir_all(effective_path.parent().unwrap()).unwrap();
    fs::write(effective_path, content).unwrap();
}
