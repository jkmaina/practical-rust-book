use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("=== GUESS THE NUMBER ===");
    println!("I'm thinking of a number between 1 and 100.");
    
    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    let mut attempts = 0;
    let max_attempts = 7;
    let mut previous_guesses = Vec::new();
    
    // Game difficulty settings
    let mut range_min = 1;
    let mut range_max = 100;
    
    // Main game loop
    'game_loop: loop {
        // Display attempts remaining
        let attempts_remaining = max_attempts - attempts;
        println!("\nYou have {} attempts remaining.", attempts_remaining);
        
        // Show previous guesses if any
        if !previous_guesses.is_empty() {
            println!("Previous guesses: {:?}", previous_guesses);
            println!("The number is between {} and {}.", range_min, range_max);
        }
        
        // Get user input
        println!("Enter your guess (or 'quit' to exit):");
        
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim();
        
        // Check if user wants to quit
        if input.to_lowercase() == "quit" {
            println!("Thanks for playing! The number was: {}", secret_number);
            break;
        }
        
        // Parse the guess
        let guess: u32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        // Validate the guess is in range
        if guess < 1 || guess > 100 {
            println!("Please enter a number between 1 and 100!");
            continue;
        }
        
        // Check if this guess was already made
        if previous_guesses.contains(&guess) {
            println!("You already guessed {}! Try a different number.", guess);
            continue;
        }
        
        // Record the attempt
        attempts += 1;
        previous_guesses.push(guess);
        
        println!("You guessed: {}", guess);
        
        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                // Update the range for hints
                if guess > range_min {
                    range_min = guess + 1;
                }
            },
            Ordering::Greater => {
                println!("Too big!");
                // Update the range for hints
                if guess < range_max {
                    range_max = guess - 1;
                }
            },
            Ordering::Equal => {
                println!("\n🎉 You win! 🎉");
                println!("The number was: {}", secret_number);
                println!("It took you {} attempts.", attempts);
                
                // Calculate score based on attempts
                let score = (max_attempts - attempts + 1) * 100;
                println!("Your score: {} points", score);
                
                break 'game_loop;
            }
        }
        
        // Check if out of attempts
        if attempts >= max_attempts {
            println!("\n😢 You've used all your attempts!");
            println!("The number was: {}", secret_number);
            break;
        }
        
        // Provide encouragement based on remaining attempts
        if attempts_remaining <= 2 {
            println!("Getting close to the end! Make these guesses count!");
        }
    }
    
    // Game statistics
    println!("\nGame Statistics:");
    println!("Total attempts: {}/{}", attempts, max_attempts);
    println!("Numbers guessed: {:?}", previous_guesses);
    
    // Ask to play again
    println!("\nWould you like to play again? (yes/no)");
    
    let mut play_again = String::new();
    
    io::stdin()
        .read_line(&mut play_again)
        .expect("Failed to read line");
    
    if play_again.trim().to_lowercase() == "yes" {
        println!("\nStarting a new game...");
        main(); // Recursively start a new game
    } else {
        println!("Thanks for playing! Goodbye!");
    }
}
