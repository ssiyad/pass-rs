mod args;

pub fn run() {
    let args = args::parse_args();
    let path = args
        .path
        .unwrap_or("/Users/ssiyad/.local/share/gopass/stores/root".to_string());

    match args.command {
        Some(args::Commands::List) => list::run(&path).unwrap(),
        _ => {}
    }
}
