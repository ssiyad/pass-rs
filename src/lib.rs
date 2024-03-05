mod args;

use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = args::parse_args();
    let path = args
        .path
        .unwrap_or("/Users/ssiyad/.local/share/gopass/stores/root".to_string());

    match args.command {
        Some(args::Commands::Edit) => edit::run()?,
        Some(args::Commands::List) => list::run(&path)?,
        Some(args::Commands::Show) => show::run()?,
        _ => {}
    }

    Ok(())
}
