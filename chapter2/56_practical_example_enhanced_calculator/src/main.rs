use std::io;
fn main() {
    println!("=== Enhanced Calculator ===");
    println!("This calculator can perform basic arithmetic operations.");
    println!("Enter 'q' at any prompt to quit.");
    
    loop {
        // Get first number
        println!("\nEnter the first number:");
        let first_number = match get_number_input() {
            Some(num) => num,
            None => break,
        };
        
        // Get operation
        println!("\nEnter the operation (+, -, *, /, %, ^):");
        let operation = match get_operation_input() {
            Some(op) => op,
            None => break,
        };
        
        // Get second number
        println!("\nEnter the second number:");
        let second_number = match get_number_input() {
            Some(num) => num,
            None => break,
        };
        
        // Perform calculation
        let result = calculate(first_number, operation, second_number);
        
        // Display result
        match result {
            Ok(value) => {
                println!("\nResult: {} {} {} = {}", first_number, operation, second_number, value);
                
                // Format the result based on whether it's a whole number
                if value.fract() == 0.0 {
                    println!("(Integer result: {})", value as i64);
                } else {
                    println!("(Decimal approximation: {:.6})", value);
                }
            },
            Err(message) => println!("\nError: {}", message),
        }
        
        // Ask if the user wants to perform another calculation
        println!("\nPerform another calculation? (y/n):");
        if !get_yes_no_input() {
            break;
        }
    }
    
    println!("\nThank you for using the calculator!");
}
// Function to get and validate number input
fn get_number_input() -> Option<f64> {
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        let input = input.trim();
        
        // Check if the user wants to quit
        if input.to_lowercase() == "q" {
            return None;
        }
        
        match input.parse() {
            Ok(num) => return Some(num),
            Err(_) => {
                println!("Invalid number. Please try again (or 'q' to quit):");
                continue;
            }
        }
    }
}
// Function to get and validate operation input
fn get_operation_input() -> Option<char> {
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        let input = input.trim();
        
        // Check if the user wants to quit
        if input.to_lowercase() == "q" {
            return None;
        }
        
        if input.len() != 1 {
            println!("Please enter a single character (+, -, *, /, %, ^) or 'q' to quit:");
            continue;
        }
        
        let operation = input.chars().next().unwrap();
        
        if operation == '+' || operation == '-' || operation == '*' || operation == '/' || 
           operation == '%' || operation == '^' {
            return Some(operation);
        } else {
            println!("Invalid operation. Please enter +, -, *, /, %, or ^ (or 'q' to quit):");
        }
    }
}
// Function to get yes/no input
fn get_yes_no_input() -> bool {
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        let input = input.trim().to_lowercase();
        
        if input == "y" || input == "yes" {
            return true;
        } else if input == "n" || input == "no" || input == "q" {
            return false;
        } else {
            println!("Please enter 'y' or 'n':");
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
        '%' => {
            if second == 0.0 {
                Err(String::from("Modulo by zero is not allowed"))
            } else {
                Ok(first % second)
            }
        },
        '^' => Ok(first.powf(second)),
        _ => Err(String::from("Invalid operation")),
    }
}