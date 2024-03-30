use super::tree::Node;
use ignore::Walk;
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

pub fn remove(name: String) {
    fs::remove_file(path(name)).unwrap();
}

pub fn tree(flat: bool, no_color: bool) {
    let root = crate::args::root();
    let walker = Walk::new(&root);

    for n in walker.skip(1).filter_map(|x| x.ok()) {
        let path = n.path();
        Node::new(
            path.strip_prefix(&root)
                .unwrap()
                .components()
                .map(|x| x.as_os_str().to_string_lossy().to_string())
                .collect::<Vec<String>>(),
            path.is_dir(),
        )
        .print(flat, !no_color)
        .unwrap();
    }
}
