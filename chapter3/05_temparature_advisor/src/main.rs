use std::io;
fn main() {
    println!("Welcome to the Temperature Advisor!");
    println!("Please enter the current temperature in Celsius:");
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let temperature: f32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("That's not a valid temperature! Using 22°C as default.");
            22.0
        }
    };
    
    println!("The temperature is {:.1}°C", temperature);
    
    if temperature < 0.0 {
        println!("It's freezing outside! Wear a heavy coat, gloves, and a hat.");
    } else if temperature < 10.0 {
        println!("It's quite cold. A warm coat is recommended.");
    } else if temperature < 20.0 {
        println!("It's a bit cool. A light jacket should be fine.");
    } else if temperature < 30.0 {
        println!("The weather is pleasant. Enjoy!");
    } else {
        println!("It's hot outside! Stay hydrated and seek shade.");
    }
    
    // Additional advice based on specific conditions
    if temperature > 35.0 {
        println!("Warning: Extreme heat! Consider staying indoors.");
    }
    
    if temperature < -10.0 {
        println!("Warning: Extreme cold! Limit your time outside.");
    }
}
