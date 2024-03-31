use crate::crypto;
use crate::pwgen::Generator as PwGen;
use crate::storage;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use inquire::{validator::Validation, Confirm, Password, Select, Text};
use serde::Deserialize;
use std::collections::HashMap;
use std::io;
use std::process;

#[derive(Debug, Deserialize)]
struct Attribute {
    name: String,
    prompt: String,
    prompt_type: String,
}

impl Attribute {
    /// Prompt the user for the attribute.
    fn ask(&self) -> String {
        match self.prompt_type.as_str() {
            "string" => Text::new(&self.prompt).prompt().unwrap(),
            "password" => {
                if Confirm::new("Generate Password")
                    .with_default(true)
                    .prompt()
                    .unwrap()
                {
                    let mut generator = PwGen::new();

                    // Ask if the user wants to include uppercase.
                    if Confirm::new("Uppercase")
                        .with_default(true)
                        .prompt()
                        .unwrap()
                    {
                        generator = generator.with_uppercase();
                    }

                    // Ask if the user want to include digits.
                    if Confirm::new("Digits").with_default(true).prompt().unwrap() {
                        generator = generator.with_digits();
                    }

                    // Ask if the user wants to include special chars.
                    if Confirm::new("Special chars")
                        .with_default(false)
                        .prompt()
                        .unwrap()
                    {
                        generator = generator.with_special();
                    }

                    // Ask for the length of the password.
                    let length = Text::new("Length")
                        .with_default("16")
                        .with_validator(|input: &str| match input.parse::<usize>() {
                            Ok(_) => Ok(Validation::Valid),
                            Err(_) => Ok(Validation::Invalid("Invalid number".into())),
                        })
                        .prompt()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();

                    // Prepare generator.
                    generator = generator.prepare();

                    // Generate the password and return it.
                    generator.generate(length)
                } else {
                    // Ask for password and return it.
                    Password::new("Password").prompt().unwrap()
                }
            }
            _ => panic!("Unknown prompt type"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Template {
    priority: u8,
    name: String,
    prefix: String,
    name_from: Vec<String>,
    welcome: String,
    attributes: Vec<Attribute>,
}

pub fn main() {
    // Load templates.
    let mut templates = [
        include_str!("templates/0-websites.toml"),
        include_str!("templates/1-pin.toml"),
    ]
    .iter()
    .map(|t| toml::from_str::<Template>(t).unwrap())
    .collect::<Vec<Template>>();

    // Sort templates by priority.
    templates.sort_by(|a, b| a.priority.cmp(&b.priority));

    // Prompt user for category.
    let category = Select::new(
        "Category",
        templates
            .iter()
            .map(|t| t.name.clone())
            .collect::<Vec<String>>(),
    )
    .prompt()
    .unwrap();

    // Get category handler.
    let template = templates.iter().find(|t| t.name == category).unwrap();

    // Greet the user.
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Green),
        Print(template.welcome.clone()),
        Print("\n"),
        ResetColor,
    )
    .ok();

    // Prompt user for attributes.
    let mut data_map: HashMap<String, String> = HashMap::new();
    for attribute in &template.attributes {
        data_map.insert(attribute.name.clone(), attribute.ask());
    }

    // Build content.
    let content = data_map
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<String>>()
        .join("\n");

    // Build name.
    let name = template.prefix.clone()
        + "/"
        + template
            .name_from
            .iter()
            .map(|n| data_map.get(n).unwrap().to_owned())
            .collect::<Vec<String>>()
            .join("/")
            .as_str();

    // Ask for confirmation before saving. If the user doesn't confirm, abort the operation.
    let confirm_save = Confirm::new("Save").with_default(true).prompt().unwrap();
    if !confirm_save {
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

    // Encrypt content.
    let content = crypto::get().encrypt(content);

    // Write to file.
    storage::get().write(name, content);
}
