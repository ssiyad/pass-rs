mod args;

use crate::crypto;
pub use args::Args;
use std::fs;

pub fn main(args: Args) {
    // Get the root directory.
    let root = super::args::root();

    // Create root directory if it doesn't exist.
    if !root.exists() {
        fs::create_dir(&root).unwrap();
    }

    // Initialize the crypto backend
    crypto::init(args.crypto);

    // Initialize the storage backend
    args.storage.init();
}
