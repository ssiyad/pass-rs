mod args;
mod generator;

pub use args::Args;
use crossterm::terminal;
pub use generator::Generator;

pub fn main(args: Args) {
    let mut generator = Generator::new().with_lowercase().with_uppercase();

    if args.digits {
        generator = generator.with_digits();
    }

    if args.symbols {
        generator = generator.with_special();
    }

    generator = generator.prepare();

    let (terminal_width, _) = terminal::size().unwrap_or((args.length as u16, 0));
    let per_line = terminal_width / (args.length as u16 + 1);
    let entries = per_line * 10;
    let mut counter = 0;

    for _ in 0..entries {
        print!("{} ", generator.generate(args.length));
        counter += 1;

        if counter == per_line {
            println!();
            counter = 0;
        }
    }
}
