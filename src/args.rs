use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long)]
    pub path: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List,
    Edit,
}

pub fn parse_args() -> Args {
    Args::parse()
}
