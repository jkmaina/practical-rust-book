// This example demonstrates an early exit from a for loop in Rust. 
// It searches for the first even number in an array of integers and exits the loop once found.

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    println!("Looking for the first even number...");
    
    for &number in numbers.iter() {
        if number % 2 == 0 {
            println!("Found it! The first even number is: {}", number);
            break;
        }
        println!("Checking {}...", number);
    }
}
