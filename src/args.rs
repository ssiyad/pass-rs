use std::env;
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
    Edit(crate::edit::Args),

    /// Setup the password store
    Init(crate::init::Args),

    /// List all password entries
    #[clap(alias = "ls")]
    List(crate::list::Args),

    /// Manage time based one time passwords
    Otp(crate::otp::Args),

    /// Generate a new password
    #[clap(alias = "generate")]
    Pwgen(crate::pwgen::Args),

    /// Remove an existing password entry
    #[clap(alias = "rm")]
    Remove(crate::remove::Args),

    /// Show an existing password entry
    Show(crate::show::Args),
}

/// Returns the root directory for the password store. This will be
/// `~/.pass-rs` by default.
pub fn root() -> PathBuf {
    env::home_dir()
        .expect("Unable to locate home directory")
        .join(".pass-rs")
}
