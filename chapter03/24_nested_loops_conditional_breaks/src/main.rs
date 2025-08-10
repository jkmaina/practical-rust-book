// This program demonstrates the use of nested loops and conditional breaks in Rust.
// It finds pairs of numbers that sum to a specific value and uses labels to control loop flow.

fn main() {
    // Find pairs of numbers that sum to 10
    println!("Pairs of numbers that sum to 10:");
    
    'outer: for i in 1..=5 {
        for j in 1..=5 {
            if i + j == 10 {
                println!("Found a pair: {} + {} = 10", i, j);
                continue 'outer; // Move to the next i
            }
        }
    }
}
