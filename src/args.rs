use std::env;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long)]
    root: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Args {
    pub fn root(&self) -> PathBuf {
        self.root.clone().unwrap_or_else(|| {
            let root_dir = ".password-store";
            env::home_dir().unwrap().join(root_dir)
        })
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Create,
    Edit {
        #[arg(index = 1)]
        item: String,
    },
    List,
    Show {
        #[arg(index = 1)]
        item: String,
    },
}

pub fn parse_args() -> Args {
    Args::parse()
}
