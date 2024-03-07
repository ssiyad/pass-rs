use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let arg = config::parse();

    match arg.command() {
        config::Command::Create => create::run()?,
        config::Command::Edit { .. } => edit::run()?,
        config::Command::List => list::run()?,
        config::Command::Pwgen { .. } => pwgen::run()?,
        config::Command::Show { .. } => show::run()?,
    }

    Ok(())
}
