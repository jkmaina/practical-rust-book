// This program implements a number guessing game in Rust.
// The user has 7 attempts to guess a randomly generated number between 1 and 100
// It provides feedback on whether the guess is too high, too low, or correct.
// If the user fails to guess the number within the allowed attempts, the game reveals the number
// and ends the game.

use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    
    // Generate a random number between 1 and 100
    let secret_number = rand::rng().random_range(1..=100);
    
    let mut attempts = 0;
    let max_attempts = 7;
    
    while attempts < max_attempts {
        println!("\nYou have {} attempts remaining.", max_attempts - attempts);
        println!("Please input your guess:");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Convert the guess to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue; // Skip this iteration and try again
            }
        };
        
        attempts += 1;
        
        println!("You guessed: {}", guess);
        
        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! It took you {} attempts.", attempts);
                break;
            }
        }
    }
    
    if attempts == max_attempts {
        println!("\nYou've used all your attempts. The number was: {}", secret_number);
    }
}
