use std::env;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Create,
    Edit(super::edit::Args),
    List,
    Pwgen(super::pwgen::Args),
    Setup,
    Show(super::show::Args),
}

/// Returns the root directory for the password store. This will be
/// `~/.password-store` by default.
pub fn root() -> PathBuf {
    env::home_dir().unwrap().join(".password-store")
}

/// Returns the key to use for encryption/decryption. This will be stored
/// in a file `.gpg-id` in the root directory.
pub fn key() -> String {
    let path = root().join(".gpg-id");
    fs::read_to_string(path).unwrap()
}
