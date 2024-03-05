mod args;

use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let arg = args::parse_args();
    let path = arg.path();

    match arg.command {
        Some(args::Commands::Edit { item }) => edit::run(path.join(item))?,
        Some(args::Commands::List) => list::run(path)?,
        Some(args::Commands::Show { item }) => show::run(path.join(item))?,
        _ => {}
    }

    Ok(())
}
