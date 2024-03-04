use std::error::Error;
use std::fs;

pub fn run(path: &str) -> Result<(), Box<dyn Error>> {
    list_dir(path, 0)?;
    Ok(())
}

fn list_dir(path: &str, level: u32) -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches(".gpg");
        let is_hidden = file_name.starts_with('.');
        let is_dir = path.is_dir();
        let padding = "  ".repeat(level as usize);

        if is_hidden {
            continue;
        }

        if is_dir || level > 0 {
            println!("{}{}", padding, file_name);
        }

        if is_dir {
            list_dir(path.to_str().unwrap(), level + 1)?;
        }
    }

    Ok(())
}
