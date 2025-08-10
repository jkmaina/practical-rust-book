use std::io;
fn main() {
    println!("Simple Calculator");
    println!("Type 'quit' to exit or 'clear' to reset.");
    
    let mut result = 0.0;
    
    loop {
        println!("\nCurrent result: {}", result);
        println!("Enter an operation (+, -, *, /) followed by a number:");
        
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim();
        
        // Exit condition 1
        if input == "quit" {
            println!("Exiting calculator...");
            break;
        }
        
        // Exit condition 2
        if input == "clear" {
            println!("Clearing result...");
            result = 0.0;
            continue;
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
    }
    
    println!("Final result: {}", result);
}
