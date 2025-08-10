use std::io;
fn main() {
    println!("=== Enhanced Calculator ===");
    println!("Available operations: +, -, *, /");
    println!("Special commands: clear, history, quit");
    
    let mut result = 0.0;
    let mut history = Vec::new();
    
    loop {
        println!("\nCurrent result: {}", result);
        println!("Enter an operation and number (e.g., '+ 5'), or a command:");
        
        // Get user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim();
        
        // Check for special commands
        match input.to_lowercase().as_str() {
            "quit" => {
                println!("Exiting calculator...");
                break;
            },
            "clear" => {
                println!("Clearing result...");
                result = 0.0;
                continue;
            },
            "history" => {
                if history.is_empty() {
                    println!("No calculation history yet.");
                } else {
                    println!("Calculation history:");
                    for (i, entry) in history.iter().enumerate() {
                        println!("{}. {}", i + 1, entry);
                    }
                }
                continue;
            },
            _ => {} // Not a special command, continue with calculation
        }
        
        // Parse the operation and number
        if input.len() < 2 {
            println!("Invalid input. Try again.");
            continue;
        }
        
        let operation = &input[0..1];
        let number_str = &input[1..].trim();
        
        let number: f64 = match number_str.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Try again.");
                continue;
            }
        };
        
        // Store the previous result for history
        let previous_result = result;
        
        // Perform the calculation
        match operation {
            "+" => result += number,
            "-" => result -= number,
            "*" => result *= number,
            "/" => {
                if number == 0.0 {
                    println!("Cannot divide by zero. Try again.");
                    continue;
                }
                result /= number;
            },
            _ => {
                println!("Invalid operation. Try again.");
                continue;
            }
        }
        
        // Add to history
        let history_entry = format!("{} {} {} = {}", previous_result, operation, number, result);
        history.push(history_entry);
        
        // Limit history size to last 10 entries
        if history.len() > 10 {
            history.remove(0);
        }
    }
    
    println!("Final result: {}", result);
    println!("Thanks for using the Enhanced Calculator!");
    
}