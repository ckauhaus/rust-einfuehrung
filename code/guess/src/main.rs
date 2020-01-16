use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

/// Prompts the user to guess a number given in `secret` as attempt # `attempt`.
///
/// # Returns
/// `true` if the user was right, `false` otherwise.
pub fn guess(secret: u32, attempt: usize) -> bool {
    println!("Guess a number between 0 and 9 ({}/3) > ", attempt);
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let guess = match line.trim().parse() {
        Ok(n) => n,
        Err(e) => {
            println!("Failed to recognize number: {}", e);
            return false;
        }
    };
    match secret.cmp(&guess) {
        Ordering::Equal => {
            println!("Correct!");
            return true;
        }
        Ordering::Greater => println!("Too low, try again."),
        Ordering::Less => println!("Too high, try again."),
    }
    false
}

fn main() {
    let secret = rand::thread_rng().gen_range(0, 10);
    for i in 1..4 {
        if guess(secret, i) {
            return;
        }
    }
    println!("Game over :-(")
}
