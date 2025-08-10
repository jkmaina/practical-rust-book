// This is a simple Rust program demonstrating the use of the loop construct
// to return a value from the loop.
// The loop will run until a certain condition is met, at which point it will
// break out of the loop and return a value.
// In this case, the loop will count from 1 to 10 and return the value
// of the counter multiplied by 2 when it reaches 10.

fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2; // Return counter * 2 from the loop
        }
    };
    
    println!("The result is: {}", result); // Prints: The result is: 20
}
