mod args;

use crate::crypto;
use crate::storage;
pub use args::Args;

pub fn main(args: Args) {
    let content_raw = storage::get().read(args.item);
    let content = crypto::get().decrypt(content_raw);

    if args.clip {
        let password = extract_password(&content);
        crate::clipboard::copy(password);
    } else {
        println!("{}", content);
    }
}

fn extract_password(content: &str) -> &str {
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
