mod args;
mod create;
mod edit;
mod gpg;
mod list;
mod pwgen;
mod setup;
mod show;

use clap::Parser;

pub fn main() {
    let args = args::Args::parse();

    match args.command {
        Some(args::Command::Create) => create::main(),
        Some(args::Command::Edit(options)) => edit::main(options),
        Some(args::Command::List) => list::main(),
        Some(args::Command::Pwgen(options)) => pwgen::main(options),
        Some(args::Command::Setup) => setup::main(),
        Some(args::Command::Show(options)) => show::main(options),
        _ => (),
    }
}
