// This program demonstrates a simple while loop in Rust that counts down from 3 to 1 and then prints "Liftoff!"
// It uses a mutable variable to keep track of the countdown.
// The loop continues until the variable reaches 0.
fn main() {
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("Liftoff!");
}
