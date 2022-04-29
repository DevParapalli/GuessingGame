

use rand::{
    self,
    distributions::{self, uniform::SampleUniform, Distribution},
};
use std::{
    cmp::Ordering,
    str::FromStr,
    io::stdin,
    fmt::Debug,
};

struct GuessProvider<T: SampleUniform> {
    rng: rand::rngs::ThreadRng,
    distribution: distributions::Uniform<T>,
}

struct GuessProviderBuilder<T: SampleUniform> {
    rng: Option<rand::rngs::ThreadRng>,
    distribution: Option<distributions::Uniform<T>>
}

impl<T: SampleUniform> GuessProvider<T> {
    fn provide_guess(&mut self) -> T {
        self.distribution.sample(&mut self.rng)
    }
}

impl<T: SampleUniform> Default for GuessProviderBuilder<T> {
    fn default() -> Self {
        Self {
            rng: Default::default(),
            distribution: Default::default(),
        }
    }
}

impl<T: SampleUniform> GuessProviderBuilder<T> {
    fn init_rng(mut self) -> Self {
        self.rng = Some(rand::thread_rng());
        self
    }

    fn set_uniform_distribution(mut self, low: T, high: T) -> Self {
        self.distribution = Some(distributions::Uniform::new_inclusive(low, high));
        self
    }

    fn build(mut self) -> GuessProvider<T> {
        if self.rng.is_none() || self.distribution.is_none() {
            panic!("all components are not initiated properly");
        }
        GuessProvider {
            rng: self.rng.take().unwrap(),
            distribution: self.distribution.take().unwrap(),
        }
    }
}

struct InputHandler {}

impl Default for InputHandler {
    fn default() -> Self {
        Self {}
    }
}

impl InputHandler {
    fn provide_guess<T: FromStr + Debug>(&self) -> T {
        let mut o;
        loop {
            let mut s = String::new();
            stdin().read_line(&mut s).expect("Failed read");
            o = s.strip_suffix("\n").unwrap_or(&s).parse().ok();
            if o.is_some() {
                break;
            } else {
                println!("could not parse, enter again: ");
            }
        }
        o.unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
enum Difficulty {
    Baby = 1,
    Im14 = 5,
    Virgin = 14,
    Chad = 35,
    GigaChad = 63,
}

fn main() {
    let input = InputHandler::default();
    let custom_difficulty = false;
    let difficulty = if custom_difficulty {
        println!("Choose Difficulty: Enter 1/2/3/4/5: ");
        let diff;
        loop {
            let d = match input.provide_guess::<u8>() {
                1 => Some(Difficulty::Baby),
                2 => Some(Difficulty::Im14),
                3 => Some(Difficulty::Virgin),
                4 => Some(Difficulty::Chad),
                5 => Some(Difficulty::GigaChad),
                _ => None
            };
            if d.is_none() {
                println!("incorrect difficulty, choose again: ");
            } else {
                diff = d.unwrap();
                println!("you choose {diff:?} difficulty, your range is [0, {}]", 1 << diff as u64);
                break;
            }
        }
        diff
    } else {
        Difficulty::Im14
    };
    let mut rng = GuessProviderBuilder::default()
        .init_rng()
        .set_uniform_distribution(0u64, 1 << difficulty as u64)
        .build();
    let computer_guess = rng.provide_guess();
    loop {
        println!("Enter a guess: ");
        let player_guess = input.provide_guess::<u64>();
        match player_guess.cmp(&computer_guess) {
            Ordering::Equal => {
                println!("You have successfully guessed the number");
                break;
            }
            Ordering::Greater => {
                println!("Guess is Higher than N.");
            }
            Ordering::Less => {
                println!("Guess is Lower than N.");
            }
        }
    }
}
