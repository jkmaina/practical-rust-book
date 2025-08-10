use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};

fn main() {
    println!("=== Enhanced Calculator ===");
    println!("Available operations: +, -, *, /, %, ^, sqrt");
    println!("Memory commands: M+, M-, MR, MC");
    println!("Special commands: clear, history, save, load, quit");

    let mut result = 0.0;
    let mut history = Vec::new();
    let mut memory = 0.0;

    loop {
        println!("\nCurrent result: {}", result);
        println!("Enter an operation and number (e.g., '+ 5'), 'sqrt', or a command:");

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
            "save" => {
                match File::create("history.txt") {
                    Ok(mut file) => {
                        for entry in &history {
                            writeln!(file, "{}", entry).unwrap();
                        }
                        println!("History saved to history.txt");
                    }
                    Err(e) => println!("Failed to save history: {}", e),
                }
                continue;
            },
            "load" => {
                match File::open("history.txt") {
                    Ok(file) => {
                        history.clear();
                        for line in BufReader::new(file).lines() {
                            if let Ok(entry) = line {
                                history.push(entry);
                            }
                        }
                        println!("History loaded from history.txt");
                    }
                    Err(e) => println!("Failed to load history: {}", e),
                }
                continue;
            },
            "mr" => {
                println!("Memory recall: {}", memory);
                result = memory;
                continue;
            },
            "mc" => {
                println!("Memory cleared.");
                memory = 0.0;
                continue;
            },
            _ => {} // Not a special command, continue with calculation
        }

        // Memory add/subtract (M+, M-)
        if input.to_uppercase().starts_with("M+") {
            let num_str = input[2..].trim();
            let num: f64 = match num_str.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid number for M+.");
                    continue;
                }
            };
            memory += num;
            println!("Added {} to memory. Memory now: {}", num, memory);
            continue;
        }
        if input.to_uppercase().starts_with("M-") {
            let num_str = input[2..].trim();
            let num: f64 = match num_str.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid number for M-.");
                    continue;
                }
            };
            memory -= num;
            println!("Subtracted {} from memory. Memory now: {}", num, memory);
            continue;
        }

        // Handle sqrt as a special case (no number needed)
        if input == "sqrt" {
            if result < 0.0 {
                println!("Cannot take square root of a negative number.");
                continue;
            }
            let previous_result = result;
            result = result.sqrt();
            let history_entry = format!("sqrt({}) = {}", previous_result, result);
            history.push(history_entry);
            if history.len() > 10 {
                history.remove(0);
            }
            continue;
        }

        // Allow calculation with result of a specific history entry: use syntax "#3 + 5"
        if input.starts_with('#') {
            let parts: Vec<&str> = input[1..].split_whitespace().collect();
            if parts.len() < 2 {
                println!("Invalid history reference. Use format: #<n> <operation> <number>");
                continue;
            }
            let idx: usize = match parts[0].parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid history entry number.");
                    continue;
                }
            };
            if idx == 0 || idx > history.len() {
                println!("History entry out of range.");
                continue;
            }
            // Try to extract the result from the history entry
            let hist_entry = &history[idx - 1];
            let hist_result: f64 = match hist_entry.split('=').last().unwrap().trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Could not parse result from history entry.");
                    continue;
                }
            };
            // Now parse the operation and number
            let op = &parts[1][0..1];
            let num_str = &parts[1][1..].trim();
            let num: f64 = match num_str.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid number after operation.");
                    continue;
                }
            };
            let mut temp_result = hist_result;
            match op {
                "+" => temp_result += num,
                "-" => temp_result -= num,
                "*" => temp_result *= num,
                "/" => {
                    if num == 0.0 {
                        println!("Cannot divide by zero.");
                        continue;
                    }
                    temp_result /= num;
                },
                "%" => {
                    if num == 0.0 {
                        println!("Cannot modulo by zero.");
                        continue;
                    }
                    temp_result %= num;
                },
                "^" => temp_result = temp_result.powf(num),
                _ => {
                    println!("Invalid operation after history entry.");
                    continue;
                }
            }
            let history_entry = format!("{} {} {} = {}", hist_result, op, num, temp_result);
            history.push(history_entry);
            if history.len() > 10 {
                history.remove(0);
            }
            result = temp_result;
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
            "%" => {
                if number == 0.0 {
                    println!("Cannot modulo by zero. Try again.");
                    continue;
                }
                result %= number;
            },
            "^" => {
                result = result.powf(number);
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