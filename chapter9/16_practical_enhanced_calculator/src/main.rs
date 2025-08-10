use std::io;

#[derive(Debug)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
    Power(f64, f64),
    Sqrt(f64),
}
#[derive(Debug)]
enum CalculationError {
    DivisionByZero,
    InvalidOperation,
    Overflow,
    NegativeSquareRoot,
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
        Operation::Power(a, b) => {
            let result = a.powf(b);
            if result.is_infinite() || result.is_nan() {
                Err(CalculationError::Overflow)
            } else {
                Ok(result)
            }
        },
        Operation::Sqrt(a) => {
            if a < 0.0 {
                Err(CalculationError::NegativeSquareRoot)
            } else {
                Ok(a.sqrt())
            }
        },
    }
}
fn parse_operation(input: &str) -> Result<Operation, CalculationError> {
    let input = input.trim();
    
    // Handle sqrt function
    if input.starts_with("sqrt") {
        let num_str = input[4..].trim();
        let a = num_str.parse::<f64>().map_err(|_| CalculationError::InvalidOperation)?;
        return Ok(Operation::Sqrt(a));
    }
    
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
            "^" => Ok(Operation::Power(a, b)),
            _ => Err(CalculationError::InvalidOperation),
        };
    }
    
    // Try mixed spacing format (e.g., "1+ 2", "5^3")
    for (i, ch) in input.char_indices() {
        if matches!(ch, '+' | '-' | '*' | '/' | '^') {
            let a_str = input[..i].trim();
            let b_str = input[i+1..].trim();
            
            let a = a_str.parse::<f64>().map_err(|_| CalculationError::InvalidOperation)?;
            let b = b_str.parse::<f64>().map_err(|_| CalculationError::InvalidOperation)?;
            
            return match ch {
                '+' => Ok(Operation::Add(a, b)),
                '-' => Ok(Operation::Subtract(a, b)),
                '*' => Ok(Operation::Multiply(a, b)),
                '/' => Ok(Operation::Divide(a, b)),
                '^' => Ok(Operation::Power(a, b)),
                _ => Err(CalculationError::InvalidOperation),
            };
        }
    }
    
    Err(CalculationError::InvalidOperation)
}
fn main() {
    println!("Enhanced Calculator");
    println!("Enter an operation in one of these formats:");
    println!("  '2 + 3', '2+3', '1+ 2' (flexible spacing)");
    println!("  'sqrt 9' or 'sqrt9'");
    println!("Supported operators: +, -, *, /, ^");
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
        
        if let Ok(operation) = parse_operation(input) {
            match calculate(operation) {
                Ok(result) => println!("Result: {}", result),
                Err(CalculationError::DivisionByZero) => println!("Error: Division by zero"),
                Err(CalculationError::Overflow) => println!("Error: Overflow or invalid result"),
                Err(CalculationError::NegativeSquareRoot) => println!("Error: Cannot take square root of a negative number"),
                Err(CalculationError::InvalidOperation) => println!("Error: Invalid operation"),
            }
        } else {
            println!("Error: Invalid input format");
        }
    }
    
    println!("Goodbye!");
}
 
