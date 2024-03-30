mod args;

pub use args::Args;

pub fn main(args: Args) {
    crate::storage::get().tree(args.flat, args.no_color);
}
