mod args;
mod generator;

pub use args::Args;
pub use generator::Generator;

pub fn main(args: Args) {
    let mut generator = Generator::new()
        .with_lowercase()
        .with_uppercase()
        .with_digits()
        .with_special()
        .prepare();

    for _ in 0..10 {
        println!("{}", generator.generate(args.length));
    }
}
