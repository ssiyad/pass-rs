use std::fs;
use std::path::PathBuf;

pub fn main() {
    let root = super::args::root();
    list_dir(root, 0);
}

fn list_dir(path: PathBuf, level: usize) {
    let entries = fs::read_dir(path).expect("Failed to read directory");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        let file_name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches(".gpg");
        let is_hidden = file_name.starts_with('.');
        let is_dir = path.is_dir();
        let padding = "  ".repeat(level);

        if is_hidden {
            continue;
        }

        if is_dir || level > 0 {
            println!("{}{}", padding, file_name);
        }

        if is_dir {
            list_dir(path, level + 1);
        }
    }
}
