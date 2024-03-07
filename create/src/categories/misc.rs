use super::Category;
use inquire::{Password, Text};
use std::error::Error;
use std::path::PathBuf;

pub struct Misc;

impl Category for Misc {
    fn welcome(&self) -> &'static str {
        "Creating Misc login"
    }

    fn prefix(&self) -> &'static str {
        "misc"
    }

    fn prompt(&self) -> Result<(PathBuf, String), Box<dyn Error>> {
        self.greet();
        let name = Text::new("Name").prompt()?;
        let password = Password::new("Password for the Website").prompt()?;
        let comment = Text::new("Comment")
            .prompt_skippable()?
            .unwrap_or("".to_string());
        let prefix = self.prefix();
        let path = PathBuf::new().join(prefix).join(&name);
        let content = format!(
            "Password: {}\nName: {}\nComment: {}",
            password, name, comment
        );
        Ok((path, content))
    }
}
