use crate::crypto::Backend as CryptoBackend;
use crate::storage::Backend as StorageBackend;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, value_enum, default_value = "gpg")]
    pub crypto: CryptoBackend,

    #[arg(short, long, value_enum, default_value = "fs")]
    pub storage: StorageBackend,
}
