use std::io;
fn main() {
    println!("=== Temperature Converter ===");
    println!("Convert temperatures between Fahrenheit and Celsius.");    
    
    loop {
        // Get conversion type
        println!("\nSelect conversion type:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Quit");
        
        let choice = get_menu_choice();
        
        if choice == 3 {
            break;
        }
        
        // Get temperature input
        let unit = if choice == 1 { "Fahrenheit" } else { "Celsius" };
        println!("\nEnter temperature in {}:", unit);
        let temperature = get_temperature_input();
        
        // Perform conversion
        let (converted_temp, from_unit, to_unit) = if choice == 1 {
            let celsius = fahrenheit_to_celsius(temperature);
            (celsius, "Fahrenheit", "Celsius")
        } else {
            let fahrenheit = celsius_to_fahrenheit(temperature);
            (fahrenheit, "Celsius", "Fahrenheit")
        };
        
        // Display result
        println!("\n{:.2}° {} = {:.2}° {}", temperature, from_unit, converted_temp, to_unit);
        
        // Display additional information
        if choice == 1 {
            // For Fahrenheit to Celsius
            if converted_temp <= 0.0 {
                println!("Water freezes at this temperature.");
            } else if converted_temp >= 100.0 {
                println!("Water boils at this temperature.");
            }
        } else {
            // For Celsius to Fahrenheit
            if converted_temp <= 32.0 {
                println!("Water freezes at this temperature.");
            } else if converted_temp >= 212.0 {
                println!("Water boils at this temperature.");
            }
        }
        
        // Ask if they want to continue
        println!("\nPerform another conversion? (y/n):");
        if !get_yes_no_input() {
            break;
        }
    }
    
    println!("\nThank you for using the Temperature Converter!");
}
fn get_menu_choice() -> u32 {
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        let input = input.trim();
        
        match input.parse() {
            Ok(num) if num >= 1 && num <= 3 => return num,
            Ok(_) => {
                println!("Please enter a number between 1 and 3:");
            },
            Err(_) => {
                println!("Please enter a valid number:");
            }
        }
    }
}
fn get_temperature_input() -> f64 {
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        let input = input.trim();
        
        match input.parse() {
            Ok(temp) => return temp,
            Err(_) => {
                println!("Invalid temperature. Please enter a number:");
            }
        }
    }
}
fn get_yes_no_input() -> bool {
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        let input = input.trim().to_lowercase();
        
        if input == "y" || input == "yes" {
            return true;
        } else if input == "n" || input == "no" {
            return false;
        } else {
            println!("Please enter 'y' or 'n':");
        }
    }
}
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
