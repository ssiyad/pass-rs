use super::generator::Generator;
use crate::crypto;
use clap::Parser;
use crossterm::{
    cursor::MoveUp,
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
pub struct Args {
    #[arg(index = 1)]
    pub name: String,
}

pub fn main(args: Args) {
    let path = crate::args::root().join(args.name);
    let content_raw = fs::read(path).expect("Could not read file");
    let content = crypto::get().decrypt(content_raw);
    let token = content
        .lines()
        .find(|line| line.starts_with("totp: "))
        .expect("No Otp line found")
        .trim_start_matches("totp: ")
        .to_string();
    let otp_gen = Generator::new(token);

    loop {
        let code = otp_gen.generate_current();
        let refresh_in = otp_gen.refresh_current_in();

        execute!(
            io::stdout(),
            MoveUp(2),
            Clear(ClearType::FromCursorDown),
            SetForegroundColor(Color::Red),
            SetAttribute(Attribute::Bold),
            Print("Refreshing in: "),
            SetAttribute(Attribute::Reset),
            SetForegroundColor(Color::Green),
            Print(refresh_in),
            Print('\n'),
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Red),
            Print("Code: "),
            SetAttribute(Attribute::Reset),
            SetForegroundColor(Color::Green),
            Print(code),
            Print('\n'),
            ResetColor,
        )
        .ok();

        thread::sleep(Duration::from_secs(1));
    }
}
