
// A simple Rust program to create a user profile by collecting various inputs from the user.
// It demonstrates basic input handling, type conversion, and conditional logic.
use std::io;
fn main() {
    println!("=== User Profile Creator ===");
    
    // Get name
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");
    let name = name.trim();
    
    // Get age
    println!("Enter your age:");
    let mut age_input = String::new();
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read age");
    
    let age: u32 = match age_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid age entered. Using default age of 0.");
            0
        }
    };
    
    // Get email
    println!("Enter your email:");
    let mut email = String::new();
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read email");
    let email = email.trim();
    
    // Get employment status
    println!("Are you currently employed? (yes/no):");
    let mut employed_input = String::new();
    io::stdin()
        .read_line(&mut employed_input)
        .expect("Failed to read employment status");
    
    let is_employed = employed_input.trim().to_lowercase() == "yes";
    
    // Get height in centimeters
    println!("Enter your height in centimeters:");
    let mut height_input = String::new();
    io::stdin()
        .read_line(&mut height_input)
        .expect("Failed to read height");
    
    let height: f64 = match height_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid height entered. Using default height of 170.0 cm.");
            170.0
        }
    };
    
    // Display the profile
    println!("\n=== User Profile ===");
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Email: {}", email);
    println!("Employed: {}", if is_employed { "Yes" } else { "No" });
    println!("Height: {:.1} cm", height);
    
    // Calculate some derived information
    let height_in_meters = height / 100.0;
    println!("Height in meters: {:.2} m", height_in_meters);
    
    let birth_year = 2025 - age;
    println!("Approximate birth year: {}", birth_year);
    
    let is_adult = age >= 18;
    println!("Adult: {}", if is_adult { "Yes" } else { "No" });
}
