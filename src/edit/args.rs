use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(index = 1)]
    pub item: String,
}
