use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, default_value = "false")]
    pub clip: bool,

    #[arg(index = 1)]
    pub item: String,
}
