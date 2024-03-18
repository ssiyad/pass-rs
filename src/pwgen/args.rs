use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Desired length of the password
    #[arg(index = 1, default_value = "16")]
    pub length: usize,

    /// Include digits
    #[arg(short, long, default_value = "false")]
    pub digits: bool,

    /// Include special characters
    #[arg(short, long, default_value = "false")]
    pub symbols: bool,
}
