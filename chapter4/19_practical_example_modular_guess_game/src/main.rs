// A simple number guessing game in Rust.
// The user has to guess a randomly generated number between 1 and 100.
// The game provides feedback on whether the guess is too high, too low, or correct.
// The code is modularized into functions for better organization and readability.
// The game continues until the user guesses the correct number.

use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    
    let secret_number = generate_secret_number();
    
    loop {
        let guess = get_user_guess();
        
        if process_guess(guess, secret_number) {
            break;
        }
    }
}
/// Generates a random number between 1 and 100.
///
/// # Returns
///
/// A random u32 between 1 and 100.
fn generate_secret_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}
/// Gets a guess from the user.
///
/// # Returns
///
/// The user's guess as a u32.
fn get_user_guess() -> u32 {
    loop {
        println!("Please input your guess:");
        
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
    }
}
/// Processes a guess and provides feedback.
///
/// # Arguments
///
/// * `guess` - The user's guess
/// * `secret_number` - The number to guess
///
/// # Returns
///
/// `true` if the guess is correct, `false` otherwise.
fn process_guess(guess: u32, secret_number: u32) -> bool {
    println!("You guessed: {}", guess);
    
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            false
        },
        Ordering::Greater => {
            println!("Too big!");
            false
        },
        Ordering::Equal => {
            println!("You win!");
            true
        }
    }
}