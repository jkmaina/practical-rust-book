// The underscore (_) is a catch-all pattern that matches any value
// The unit value () means "do nothing" - it's an empty tuple

fn main() {
    let some_u8_value = 0u8;  // u8 can hold values 0-255
    
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),  // Catch-all: matches any other value (0, 2, 4, 6, 8-255, etc.)
    }
    
    // Example with different values
    let values = [0, 1, 2, 3, 4, 5, 800, 6, 7, 8, 9, 10];
    for value in values {
        match value {
            1 => println!("{} is one", value),
            3 => println!("{} is three", value),
            5 => println!("{} is five", value),
            7 => println!("{} is seven", value),
            _ => println!("{} is something else", value),  // Handle all other cases
        }
    }
}