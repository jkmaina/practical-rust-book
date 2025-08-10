use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("=== Guess the Number Game ===");
    
    let mut play_again = true;
    let mut total_games = 0;
    let mut total_guesses = 0;
    
    while play_again {
        let (won, guesses) = play_game();
        
        if won {
            total_games += 1;
            total_guesses += guesses;
            
            println!("You won in {} guesses!", guesses);
            println!("Average guesses per game: {:.2}", total_guesses as f64 / total_games as f64);
        }
        
        play_again = ask_play_again();
    }
    
    println!("Thanks for playing!");
}
/// Plays a single game of Guess the Number.
///
/// # Returns
///
/// A tuple containing:
/// - A boolean indicating whether the player won
/// - The number of guesses made
fn play_game() -> (bool, u32) {
    println!("\nI'm thinking of a number between 1 and 100...");
    
    let secret_number = generate_secret_number();
    let max_guesses = 10;
    let mut guesses_made = 0;
    
    while guesses_made < max_guesses {
        let remaining = max_guesses - guesses_made;
        println!("\nYou have {} guesses remaining.", remaining);
        
        let guess = get_user_guess();
        guesses_made += 1;
        
        if process_guess(guess, secret_number) {
            return (true, guesses_made);
        }
    }
    
    println!("\nYou've run out of guesses! The number was {}.", secret_number);
    (false, guesses_made)
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
        println!("Please input your guess (1-100):");
        
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse() {
            Ok(num) if num >= 1 && num <= 100 => return num,
            Ok(_) => {
                println!("Please enter a number between 1 and 100!");
                continue;
            },
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
/// Asks the user if they want to play again.
///
/// # Returns
///
/// `true` if the user wants to play again, `false` otherwise.
fn ask_play_again() -> bool {
    loop {
        println!("\nWould you like to play again? (y/n)");
        
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Please enter 'y' or 'n'.");
                continue;
            }
        }
    }
}
