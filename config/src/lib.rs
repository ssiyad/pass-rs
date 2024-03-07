use std::env;
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
        if self.key.is_some() {
            return self.key.clone();
        }
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
    Show {
        #[arg(index = 1)]
        item: String,
    },
}

pub fn parse() -> Args {
    Args::parse()
}
