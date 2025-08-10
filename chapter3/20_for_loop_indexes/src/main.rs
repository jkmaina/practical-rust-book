// This program demonstrates how to use a for loop with indexes in Rust.
// It iterates over an array of fruits and prints each fruit with its index.
// The index starts from 1 for better readability.
fn main() {
    let fruits = ["apple", "banana", "cherry", "date", "elderberry"];
    
    for (index, fruit) in fruits.iter().enumerate() {
        println!("Fruit #{}: {}", index + 1, fruit);
    }
}
