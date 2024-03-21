use super::generator::Generator;
use crate::gpg;
use clap::Parser;
use crossterm::{
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
};
use std::io;

#[derive(Parser)]
pub struct Args {
    #[arg(index = 1)]
    pub name: String,
}

pub fn main(args: Args) {
    let path = crate::args::root().join(args.name);
    let secret = gpg::decrypt(path).expect("Failed to decrypt");
    let token = secret
        .lines()
        .find(|line| line.starts_with("totp: "))
        .expect("No Otp line found")
        .trim_start_matches("totp: ")
        .to_string();
    let otp_gen = Generator::new(token);
    let code = otp_gen.generate_current();

    execute!(
        io::stdout(),
        SetAttribute(Attribute::Bold),
        SetForegroundColor(Color::Red),
        Print("Code: "),
        SetForegroundColor(Color::Green),
        Print(code),
        Print("\n"),
        ResetColor,
        SetAttribute(Attribute::Reset),
    )
    .ok();
}
