// Returning Multiple Values with Tuples in Rust
// This program demonstrates how to return multiple values from a function using tuples.
// It defines a function that calculates the sum, minimum, and maximum of an array of integers.
// The function returns these values as a tuple, which is then unpacked in the main function.

fn get_statistics(numbers: &[i32]) -> (i32, i32, i32) {
    let sum: i32 = numbers.iter().sum();
    let min: i32 = *numbers.iter().min().unwrap_or(&0);
    let max: i32 = *numbers.iter().max().unwrap_or(&0);
    
    (sum, min, max)
}
fn main() {
    let numbers = [5, 2, 9, 1, 8, 3];
    let (sum, min, max) = get_statistics(&numbers);
    
    println!("Sum: {}", sum);
    println!("Minimum: {}", min);
    println!("Maximum: {}", max);
}
