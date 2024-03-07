use std::collections::HashMap;
use std::error::Error;

type Handler = Box<dyn Fn() -> Result<(), Box<dyn Error>>>;

pub fn main() {
    let handlers = get_handlers();
    let command = config::parse().command().to_string().to_lowercase();

    if let Some(handler) = handlers.get(command.as_str()) {
        match handler() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    };
}

fn get_handlers() -> HashMap<&'static str, Handler> {
    let mut h: HashMap<&str, Handler> = HashMap::new();

    h.insert("create", Box::new(create::run));
    h.insert("edit", Box::new(edit::run));
    h.insert("list", Box::new(list::run));
    h.insert("pwgen", Box::new(pwgen::run));
    h.insert("setup", Box::new(setup::run));
    h.insert("show", Box::new(show::run));

    h
}
