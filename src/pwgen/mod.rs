mod args;
mod generator;

pub use args::Args;
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

    for _ in 0..10 {
        println!("{}", generator.generate(args.length));
    }
}
