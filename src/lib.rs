mod args;

use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let arg = args::parse_args();
    let root = arg.root();

    match arg.command {
        Some(args::Commands::Create) => create::run(root)?,
        Some(args::Commands::Edit { item }) => edit::run(root.join(item))?,
        Some(args::Commands::List) => list::run(root)?,
        Some(args::Commands::Show { item }) => show::run(root.join(item))?,
        _ => {}
    }

    Ok(())
}
