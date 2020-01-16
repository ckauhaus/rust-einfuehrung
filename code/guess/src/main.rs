use rand::prelude::*;
use std::io;

/// Prompts the user to guess a number given in `secret` as attempt # `attempt`.
///
/// # Returns
/// `true` if the user was right, `false` otherwise.
pub fn guess(secret: u32, attempt: usize) -> bool {
    println!("Guess a number between 0 and 9 (attempt {}/3) > ", attempt);
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let guess: u32 = match line.trim().parse() {
        Ok(n) => n,
        Err(e) => {
            println!("Failed to recognize number: {}", e);
            return false;
        }
    };
    if guess > secret {
        println!("Too low!");
        false
    } else if guess < secret {
        println!("Too high!");
        false
    } else {
        println!("Correct!");
        true
    }
}

fn main() {
    let secret = rand::thread_rng().gen_range(0, 10);
    for i in 1..4 {
        if guess(secret, i) {
            return;
        }
    }
    println!("GAME OVER")
}
