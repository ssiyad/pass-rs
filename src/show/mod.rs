mod args;

use super::gpg;
pub use args::Args;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub fn main(args: Args) {
    let path = super::args::root().join(args.item);
    let content = gpg::decrypt(path).expect("Failed to decrypt");

    if args.clip {
        let password = get_password(&content);
        copy(password);
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

fn copy(content: &str) {
    ClipboardContext::new()
        .expect("Failed to create clipboard context")
        .set_contents(content.to_string())
        .expect("Failed to set clipboard contents");
}
