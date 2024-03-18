mod categories;

use super::gpg;
use categories::Category;
use inquire::Select;
use std::collections::HashMap;
use std::fs;

pub fn main() {
    let categories = get_categories();

    // Prompt user for category
    let category = Select::new("Category", categories.keys().collect())
        .prompt()
        .unwrap();

    if let Some(handler) = categories.get(category) {
        let (path, content) = handler.prompt().unwrap();
        let effective_path = super::args::root().join(path);

        // Create missing directories. Skip first element, which will be the
        // file itself.
        if let Some(parent) = effective_path.ancestors().skip(1).next() {
            fs::create_dir_all(parent).expect("Failed to create directory");
        }

        // Encrypt password and write to file.
        gpg::encrypt(effective_path, content).expect("Failed to encrypt");
    }
}

fn get_categories() -> HashMap<&'static str, Box<dyn Category>> {
    let mut c: HashMap<&'static str, Box<dyn Category>> = HashMap::new();
    c.insert("Website", Box::new(categories::website::Website));
    c.insert("Misc", Box::new(categories::misc::Misc));
    c.insert("PIN", Box::new(categories::pin_code::PinCode));
    c
}