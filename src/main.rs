mod args;
mod create;
mod edit;
mod gpg;
mod list;
mod otp;
mod pwgen;
mod setup;
mod show;

use args::{Args, Cmd};
use clap::Parser;

pub fn main() {
    let args = Args::parse();

    match args.command {
        Cmd::Create => create::main(),
        Cmd::Edit(options) => edit::main(options),
        Cmd::List => list::main(),
        Cmd::Otp(options) => otp::main(options),
        Cmd::Pwgen(options) => pwgen::main(options),
        Cmd::Setup => setup::main(),
        Cmd::Show(options) => show::main(options),
    }
}
