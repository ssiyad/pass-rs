pub mod misc;
pub mod pin_code;
pub mod website;

use std::error::Error;
use std::path::PathBuf;

pub trait Category {
    /// Message to show when the category is selected.
    fn welcome(&self) -> &'static str;

    /// Greet the user, with welcome message.
    fn greet(&self) {
        println!("{}", self.welcome());
    }

    /// Prefix to use in the path.
    fn prefix(&self) -> &'static str;

    /// Prompts the user for the required attributes. Returns the path
    /// and the content to write to the file.
    fn prompt(&self) -> Result<(PathBuf, String), Box<dyn Error>>;
}
