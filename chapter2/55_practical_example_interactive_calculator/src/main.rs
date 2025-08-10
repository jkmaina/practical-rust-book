// This program is a simple interactive calculator that allows users to perform basic arithmetic operations.
// It demonstrates input handling, type conversion, and error handling in Rust.
// It prompts the user for two numbers and an operation, performs the calculation, and displays the result.
// It also includes error handling for invalid inputs and division by zero.
// It is a practical example of how to create a user-friendly command-line application in Rust.

use std::io;
fn main() {
    println!("=== Simple Calculator ===");
    println!("This calculator can perform basic arithmetic operations.");
    
    // Get first number
    println!("\nEnter the first number:");
    let first_number = get_number_input();
    
    // Get operation
    println!("\nEnter the operation (+, -, *, /):");
    let operation = get_operation_input();
    
    // Get second number
    println!("\nEnter the second number:");
    let second_number = get_number_input();
    
    // Perform calculation
    let result = calculate(first_number, operation, second_number);
    
    // Display result
    match result {
        Ok(value) => println!("\nResult: {} {} {} = {}", first_number, operation, second_number, value),
        Err(message) => println!("\nError: {}", message),
    }
}
// Function to get and validate number input
fn get_number_input() -> f64 {
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid number. Please try again:");
                continue;
            }
        }
    }
}
// Function to get and validate operation input
fn get_operation_input() -> char {
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        let input = input.trim();
        
        if input.len() != 1 {
            println!("Please enter a single character (+, -, *, /):");
            continue;
        }
        
        let operation = input.chars().next().unwrap();
        
        if operation == '+' || operation == '-' || operation == '*' || operation == '/' {
            return operation;
        } else {
            println!("Invalid operation. Please enter +, -, *, or /:");
        }
    }
}
// Function to perform the calculation
fn calculate(first: f64, operation: char, second: f64) -> Result<f64, String> {
    match operation {
        '+' => Ok(first + second),
        '-' => Ok(first - second),
        '*' => Ok(first * second),
        '/' => {
            if second == 0.0 {
                Err(String::from("Division by zero is not allowed"))
            } else {
                Ok(first / second)
            }
        },
        _ => Err(String::from("Invalid operation")),
    }
}

