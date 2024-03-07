use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long)]
    root: Option<PathBuf>,

    #[arg(short, long)]
    key: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

impl Args {
    /// Returns the root directory for the password store. This will be
    /// `~/.password-store` by default.
    pub fn root(&self) -> PathBuf {
        self.root.clone().unwrap_or_else(|| {
            let root_dir = ".password-store";
            env::home_dir().unwrap().join(root_dir)
        })
    }

    /// Returns the key to use for encryption/decryption. This will be stored
    /// in a file `.gpg-id` in the root directory.
    pub fn key(&self) -> Option<String> {
        // If `key` is provided as a command line argument, pick that
        if self.key.is_some() {
            return self.key.clone();
        }

        // Fetch `key` from `root_dir/.gpg-id`
        let path = self.root().join(".gpg-id");
        if path.exists() {
            return fs::read_to_string(path).ok();
        }

        None
    }

    pub fn command(self) -> Command {
        self.command.unwrap()
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Create,
    Edit {
        #[arg(index = 1)]
        item: String,
    },
    List,
    Pwgen {
        #[arg(index = 1, default_value = "16")]
        length: usize,
    },
    Setup,
    Show {
        #[arg(index = 1)]
        item: String,
    },
}

// This is required so that we can convert sub-commands to a string. This
// string can then be used as key to store different handlers. Since each
// handler make use of `config` directly, we don't have to pass options
// via function parameters
impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Get config. This will include both command line arguments as well as
/// global configuration.
pub fn parse() -> Args {
    Args::parse()
}
