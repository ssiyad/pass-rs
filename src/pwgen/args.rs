use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(index = 1, default_value = "16")]
    pub length: usize,
}
