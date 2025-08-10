// Simple calculator that handles basic arithmetic operations (+, -, *, /)
// Supports flexible input formats: "5 + 3", "5+3", "1+ 2", etc.
// Uses Result<T, E> for error handling and pattern matching

use std::io;

#[derive(Debug)]
enum CalculationError {
    DivisionByZero,
    InvalidOperation,
    Overflow,
}

#[derive(Debug)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn parse_operation(input: &str) -> Result<Operation, CalculationError> {
    let input = input.trim();
    
    // Try space-separated format first
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() == 3 {
        let a = parts[0].parse::<f64>().map_err(|_| CalculationError::InvalidOperation)?;
        let b = parts[2].parse::<f64>().map_err(|_| CalculationError::InvalidOperation)?;
        return match parts[1] {
            "+" => Ok(Operation::Add(a, b)),
            "-" => Ok(Operation::Subtract(a, b)),
            "*" => Ok(Operation::Multiply(a, b)),
            "/" => Ok(Operation::Divide(a, b)),
            _ => Err(CalculationError::InvalidOperation),
        };
    }
    
    // Try mixed spacing format (e.g., "1+ 2", "5+3")
    for (i, ch) in input.char_indices() {
        if matches!(ch, '+' | '-' | '*' | '/') {
            let a_str = input[..i].trim();
            let b_str = input[i+1..].trim();
            
            let a = a_str.parse::<f64>().map_err(|_| CalculationError::InvalidOperation)?;
            let b = b_str.parse::<f64>().map_err(|_| CalculationError::InvalidOperation)?;
            
            return match ch {
                '+' => Ok(Operation::Add(a, b)),
                '-' => Ok(Operation::Subtract(a, b)),
                '*' => Ok(Operation::Multiply(a, b)),
                '/' => Ok(Operation::Divide(a, b)),
                _ => Err(CalculationError::InvalidOperation),
            };
        }
    }
    
    Err(CalculationError::InvalidOperation)
}
fn calculate(operation: Operation) -> Result<f64, CalculationError> {
    match operation {
        Operation::Add(a, b) => {
            let result = a + b;
            if result.is_infinite() {
                Err(CalculationError::Overflow)
            } else {
                Ok(result)
            }
        },
        Operation::Subtract(a, b) => {
            let result = a - b;
            if result.is_infinite() {
                Err(CalculationError::Overflow)
            } else {
                Ok(result)
            }
        },
        Operation::Multiply(a, b) => {
            let result = a * b;
            if result.is_infinite() {
                Err(CalculationError::Overflow)
            } else {
                Ok(result)
            }
        },
        Operation::Divide(a, b) => {
            if b == 0.0 {
                Err(CalculationError::DivisionByZero)
            } else {
                let result = a / b;
                if result.is_infinite() {
                    Err(CalculationError::Overflow)
                } else {
                    Ok(result)
                }
            }
        },
    }
}

fn main() {
    println!("Simple Calculator");
    println!("Enter an operation: '5 + 3' or '5+3'");
    println!("Supported operators: +, -, *, /");
    println!("Enter 'quit' to exit");
    
    loop {
        println!("\nEnter an operation:");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim();
        
        if input == "quit" {
            break;
        }
        
        match parse_operation(input) {
            Ok(operation) => {
                match calculate(operation) {
                    Ok(result) => println!("Result: {}", result),
                    Err(error) => match error {
                        CalculationError::DivisionByZero => println!("Error: Division by zero"),
                        CalculationError::Overflow => println!("Error: Overflow"),
                        CalculationError::InvalidOperation => println!("Error: Invalid operation"),
                    },
                }
            },
            Err(error) => match error {
                CalculationError::InvalidOperation => println!("Error: Invalid input format"),
                _ => println!("Error: {:#?}", error),
            },
        }
    }
    
    println!("Goodbye!");
}
