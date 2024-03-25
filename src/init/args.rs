use crate::crypto::Backend as CryptoBackend;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, value_enum, default_value = "gpg")]
    pub crypto: CryptoBackend,
}
