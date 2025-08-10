use std::io;

fn main() {
    println!("Simple Calculator");
    println!("Enter an expression (e.g., 5 + 3):");

    let input = get_user_input();

    let (first_number, operator, second_number) = match parse_expression(&input) {
        Ok(result) => result,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    match calculate(first_number, &operator, second_number) {
        Ok(result) => println!("{} {} {} = {}", first_number, operator, second_number, result),
        Err(error) => println!("Error: {}", error)
    }
}

/// Gets input from the user.
fn get_user_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

/// Parses an expression of the form "number operator number".
/// Handles input with or without spaces, e.g., "5+3", "5 +3", "5+ 3", "5 + 3".
fn parse_expression(input: &str) -> Result<(f64, String, f64), String> {
    // Remove all spaces for easier parsing
    let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    // Find the operator position
    let operator_pos = cleaned.find(|c: char| "+-*/".contains(c));
    if let Some(pos) = operator_pos {
        let (left, rest) = cleaned.split_at(pos);
        let operator = rest[0..1].to_string();
        let right = &rest[1..];

        if left.is_empty() || right.is_empty() {
            return Err(String::from("Invalid expression. Please use format: number operator number"));
        }

        let first_number: f64 = match left.parse() {
            Ok(num) => num,
            Err(_) => return Err(String::from("Invalid first number")),
        };

        let second_number: f64 = match right.parse() {
            Ok(num) => num,
            Err(_) => return Err(String::from("Invalid second number")),
        };

        Ok((first_number, operator, second_number))
    } else {
        Err(String::from("Invalid expression. Please use format: number operator number"))
    }
}

/// Performs a calculation based on two numbers and an operator.
fn calculate(first_number: f64, operator: &str, second_number: f64) -> Result<f64, String> {
    match operator {
        "+" => Ok(first_number + second_number),
        "-" => Ok(first_number - second_number),
        "*" => Ok(first_number * second_number),
        "/" => {
            if second_number == 0.0 {
                Err(String::from("Division by zero"))
            } else {
                Ok(first_number / second_number)
            }
        },
        _ => Err(String::from("Invalid operator. Please use +, -, *, or /")),
    }
}
