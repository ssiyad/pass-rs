mod args;
mod clipboard;
mod create;
mod crypto;
mod edit;
mod init;
mod list;
mod otp;
mod pwgen;
mod show;
mod storage;

use args::{Args, Cmd};
use clap::Parser;

pub fn main() {
    let args = Args::parse();

    match args.command {
        Cmd::Create => create::main(),
        Cmd::Edit(options) => edit::main(options),
        Cmd::Init(options) => init::main(options),
        Cmd::List => list::main(),
        Cmd::Otp(options) => otp::main(options),
        Cmd::Pwgen(options) => pwgen::main(options),
        Cmd::Show(options) => show::main(options),
    }
}
