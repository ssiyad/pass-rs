use crate::gpg;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(index = 1)]
    name: String,

    #[arg(index = 2)]
    code: String,
}

pub fn main(args: Args) {
    let path = crate::args::root().join(args.name);
    let secret = gpg::decrypt(&path).expect("Failed to decrypt");
    let mut secret_lines = secret
        .lines()
        .filter(|x| !x.starts_with("totp: "))
        .collect::<Vec<&str>>();
    let totp_line = format!("totp: {}", args.code);
    secret_lines.push(&totp_line);
    let secret_updated = secret_lines.join("\n");
    gpg::encrypt(path, secret_updated).expect("Failed to encrypt");
}
