mod misc;
mod pin_code;
mod website;

pub use misc::Misc;
pub use pin_code::PinCode;
pub use website::Website;

use crate::pwgen::Generator as PwGen;
use inquire::{validator::Validation, Confirm, Password, Text};
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

    /// Ask for password
    fn password(&self) -> Result<String, Box<dyn Error>> {
        if Confirm::new("Generate Password")
            .with_default(true)
            .prompt()?
        {
            let mut generator = PwGen::new();

            // Ask if the user wants to include uppercase.
            if Confirm::new("Uppercase").with_default(true).prompt()? {
                generator = generator.with_uppercase();
            }

            // Ask if the user want to include digits.
            if Confirm::new("Digits").with_default(true).prompt()? {
                generator = generator.with_digits();
            }

            // Ask if the user wants to include special chars.
            if Confirm::new("Special chars").with_default(false).prompt()? {
                generator = generator.with_special();
            }

            // Ask for the length of the password.
            let length = Text::new("Length")
                .with_default("16")
                .with_validator(|input: &str| match input.parse::<usize>() {
                    Ok(_) => Ok(Validation::Valid),
                    Err(_) => Ok(Validation::Invalid("Invalid number".into())),
                })
                .prompt()?
                .parse::<usize>()?;

            // Prepare generator.
            generator = generator.prepare();

            // Generate the password and return it.
            Ok(generator.generate(length))
        } else {
            // Ask for password and return it.
            Ok(Password::new("Password").prompt()?)
        }
    }
}
