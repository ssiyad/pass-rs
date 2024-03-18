use super::Category;
use inquire::Text;
use std::error::Error;
use std::path::PathBuf;

pub struct Website;

impl Category for Website {
    fn welcome(&self) -> &'static str {
        "Creating Website login"
    }

    fn prefix(&self) -> &'static str {
        "websites"
    }

    fn prompt(&self) -> Result<(PathBuf, String), Box<dyn Error>> {
        self.greet();
        let url = Text::new("Website URL").prompt()?;
        let username = Text::new("Username").prompt()?;
        let password = self.password()?;
        let prefix = self.prefix();
        let path = PathBuf::new().join(prefix).join(&url).join(&username);
        let content = format!(
            "Password: {}\nUsername: {}\nURL: {}",
            password, username, url
        );
        Ok((path, content))
    }
}
