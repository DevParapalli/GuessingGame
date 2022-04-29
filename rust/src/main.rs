
use rand;

struct GuessProvider {

}

impl Default for GuessProvider {
    fn default() -> Self {
        todo!()
    }
}


impl GuessProvider {
    fn provide_guess() -> u32 {
        todo!()
    }
}

struct InputHandler {
    
}

impl Default for InputHandler {
    fn default() -> Self {
        todo!()
    }
}

impl InputHandler {
    fn provide_guess() -> u32 {
        todo!()
    }
}

fn main() {

    let rng = GuessProvider::default();
    let input = InputHandler::default();
    loop {
        todo!()
        let computer_guess = rng.provide_guess();
        let player_guess = input.provide_guess();
        if player_guess == computer_guess {
            println!("You have successfully guessed the number");
            break;
        } else if player_guess > computer_guess {
            println!("Guess is Higher than N.");
        } else {
            println!("Guess is Lower than N.");
        }
    }
}