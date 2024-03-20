mod args;
mod create;
mod edit;
mod gpg;
mod list;
mod otp;
mod pwgen;
mod setup;
mod show;

use args::{Args, Command};
use clap::Parser;

pub fn main() {
    let args = Args::parse();

    match args.command {
        Some(Command::Create) => create::main(),
        Some(Command::Edit(options)) => edit::main(options),
        Some(Command::List) => list::main(),
        Some(Command::Otp(options)) => otp::main(options),
        Some(Command::Pwgen(options)) => pwgen::main(options),
        Some(Command::Setup) => setup::main(),
        Some(Command::Show(options)) => show::main(options),
        _ => (),
    }
}
