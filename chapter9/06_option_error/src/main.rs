// Demonstrates type mismatch error with Option and how to fix it
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    // This would cause a compile error:
    // let sum = x + y; // Error: cannot add `Option<i8>` to `i8`
    
    // Correct way: extract value from Option first
    let sum = match y {
        Some(value) => x + value,
        None => x, // or handle None case appropriately
    };
    
    println!("Sum: {}", sum);
}