mod args;

use crate::storage;
pub use args::Args;

pub fn main(args: Args) {
    storage::get().remove(args.name);
}
