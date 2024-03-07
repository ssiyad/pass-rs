use super::Category;
use inquire::{Password, Text};
use std::error::Error;
use std::path::PathBuf;

pub struct PinCode;

impl Category for PinCode {
    fn welcome(&self) -> &'static str {
        "Creating numerical PIN"
    }

    fn prefix(&self) -> &'static str {
        "pin"
    }

    fn prompt(&self) -> Result<(PathBuf, String), Box<dyn Error>> {
        self.greet();
        let authority = Text::new("Authority").prompt()?;
        let application = Text::new("Application").prompt()?;
        let password = Password::new("Password for the Website").prompt()?;
        let comment = Text::new("Comment")
            .prompt_skippable()?
            .unwrap_or("".to_string());
        let prefix = self.prefix();
        let path = PathBuf::new()
            .join(prefix)
            .join(&authority)
            .join(&application);
        let content = format!(
            "Password: {}\nAuthority: {}\nApplication: {}\nComment: {}",
            password, authority, application, comment
        );
        Ok((path, content))
    }
}
