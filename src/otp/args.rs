use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[arg(index = 1)]
    pub secret: PathBuf,
}
