// This program demonstrates how to access an element in an array by index.
// It will panic if the index is out of bounds.
// This is a common way to illustrate Rust's safety features regarding array access.
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    
    let index = 10;
    
    // This will cause a panic at runtime
    let element = numbers[index];
    println!("The element at index {} is: {}", index, element);
}
