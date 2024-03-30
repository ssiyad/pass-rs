use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long, short, default_value = "false")]
    pub flat: bool,

    #[arg(long, short, default_value = "false")]
    pub no_color: bool,
}
