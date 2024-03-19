mod categories;

use super::gpg;
use categories::Category;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use inquire::Select;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::process;

pub fn main() {
    let categories = get_categories();

    // Prompt user for category.
    let category = Select::new("Category", categories.keys().collect())
        .prompt()
        .unwrap();

    if let Some(handler) = categories.get(category) {
        // Greet the user.
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Green),
            Print(handler.welcome()),
            Print("\n"),
            ResetColor,
        )
        .ok();

        // Prompt user for path and content.
        let (path, content) = handler.prompt().unwrap();

        // Ask for confirmation before saving. If the user doesn't confirm,
        // abort the operation.
        if !handler.confirm_save() {
            execute!(
                io::stdout(),
                SetForegroundColor(Color::Red),
                Print("Aborted"),
                Print("\n"),
                ResetColor,
            )
            .ok();
            process::exit(0);
        }

        // Build secret path
        let effective_path = super::args::root().join(path);

        // Create missing directories. Skip first element, which will be the
        // file itself.
        if let Some(parent) = effective_path.ancestors().nth(1) {
            fs::create_dir_all(parent).expect("Failed to create directory");
        }

        // Encrypt password and write to file.
        gpg::encrypt(effective_path, content).expect("Failed to encrypt");
    }
}

fn get_categories() -> HashMap<&'static str, Box<dyn Category>> {
    let mut c: HashMap<&'static str, Box<dyn Category>> = HashMap::new();
    c.insert("Website", Box::new(categories::Website));
    c.insert("Misc", Box::new(categories::Misc));
    c.insert("PIN", Box::new(categories::PinCode));
    c
}
