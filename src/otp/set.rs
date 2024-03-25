use crate::crypto;
use clap::Parser;
use std::fs;

#[derive(Parser)]
pub struct Args {
    #[arg(index = 1)]
    name: String,

    #[arg(index = 2)]
    code: String,
}

pub fn main(args: Args) {
    let path = crate::args::root().join(args.name);
    let content_raw = fs::read(&path).expect("Failed to read file");
    let crypto_backend = crypto::get();
    let content = crypto_backend.decrypt(content_raw);
    let mut content_lines = content
        .lines()
        .filter(|x| !x.starts_with("totp: "))
        .collect::<Vec<&str>>();
    let totp_line = format!("totp: {}", args.code);
    content_lines.push(&totp_line);
    let content_updated = content_lines.join("\n");
    let content_encrypted = crypto_backend.encrypt(content_updated);
    fs::write(path, content_encrypted).expect("Failed to write file");
}
