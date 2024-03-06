use std::error::Error;

use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn run(length: usize) -> Result<(), Box<dyn Error>> {
    let mut generator = Generator::new()
        .with_lowercase()
        .with_uppercase()
        .with_digits()
        .with_special()
        .prepare();

    for _ in 0..10 {
        println!("{}", generator.generate(length));
    }

    Ok(())
}

struct Generator {
    chars: Vec<char>,
    distribution: Uniform<usize>,
    rng: StdRng,
}

impl Generator {
    fn new() -> Generator {
        Generator {
            chars: vec![],
            distribution: Uniform::new(0, 1),
            rng: StdRng::from_entropy(),
        }
    }

    fn with_lowercase(mut self) -> Self {
        for c in b'a'..=b'z' {
            self.chars.push(c as char);
        }
        self
    }

    fn with_uppercase(mut self) -> Self {
        for c in b'A'..=b'Z' {
            self.chars.push(c as char);
        }
        self
    }

    fn with_digits(mut self) -> Self {
        for c in b'0'..=b'9' {
            self.chars.push(c as char);
        }
        self
    }

    fn with_special(mut self) -> Self {
        for c in b'!'..=b'/' {
            self.chars.push(c as char);
        }
        self
    }

    fn prepare(mut self) -> Self {
        self.distribution = Uniform::new(0, self.chars.len());
        self
    }

    fn generate(&mut self, length: usize) -> String {
        let mut res = String::new();
        for _ in 0..length {
            let idx = self.rng.sample(self.distribution);
            res.push(self.chars[idx]);
        }
        res
    }
}
