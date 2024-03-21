mod args;
mod generator;
mod set;
mod show;

pub use args::{Args, Cmd};

pub fn main(args: Args) {
    match args.command {
        Cmd::Set(options) => set::main(options),
        Cmd::Show(options) => show::main(options),
    }
}
