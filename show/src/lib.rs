use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = config::parse();
    let root = config.root();
    let cmd = config.command();

    if let config::Command::Show { clip, item } = cmd {
        let path = root.join(item);
        let content = gpg::decrypt(path)?;

        if clip {
            let password = get_password(&content);
            copy(password)?;
        } else {
            println!("{}", content);
        }
    }

    Ok(())
}

fn get_password(content: &str) -> &str {
    let mut password = "";
    for line in content.lines() {
        // If the line contains a space, it's not a password. This logic
        // might not be right.
        if !line.contains(" ") {
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

fn copy(content: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(content.to_string())
}
