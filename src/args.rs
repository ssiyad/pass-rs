use std::env;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about, version, arg_required_else_help(true))]
pub struct Args {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Create a new password entry
    #[clap(alias = "new")]
    Create,

    /// Edit an existing password entry
    Edit(super::edit::Args),

    /// List all password entries
    #[clap(alias = "ls")]
    List,

    /// Manage time based one time passwords
    Otp(super::otp::Args),

    /// Generate a new password
    #[clap(alias = "generate")]
    Pwgen(super::pwgen::Args),

    /// Setup the password store
    Setup,

    /// Show an existing password entry
    Show(super::show::Args),
}

/// Returns the root directory for the password store. This will be
/// `~/.pass-rs` by default.
pub fn root() -> PathBuf {
    env::home_dir()
        .expect("Unable to locate home directory")
        .join(".pass-rs")
}

/// Returns the key to use for encryption/decryption. This will be stored
/// in a file `.gpg-id` in the root directory.
pub fn key() -> String {
    let path = root().join(".gpg-id");
    fs::read_to_string(path)
        .unwrap()
        .trim_matches('\n')
        .to_string()
}
