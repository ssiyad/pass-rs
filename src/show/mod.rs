mod args;

use crate::crypto;
pub use args::Args;
use std::fs;

pub fn main(args: Args) {
    let path = super::args::root().join(args.item);
    let content_raw = fs::read(path).expect("Failed to read file");
    let content = crypto::get().decrypt(content_raw);

    if args.clip {
        let password = get_password(&content);
        crate::clipboard::copy(password);
    } else {
        println!("{}", content);
    }
}

fn get_password(content: &str) -> &str {
    let mut password = "";
    for line in content.lines() {
        // If the line contains a space, it's not a password. This logic
        // might not be right.
        if !line.contains(' ') {
            password = line;
            break;
        }

        // If the line starts with `Password: `, then it's a password.
        if line.starts_with("Password: ") {
            password = line.trim_start_matches("Password: ");
            break;
        }
    }
    password
}
