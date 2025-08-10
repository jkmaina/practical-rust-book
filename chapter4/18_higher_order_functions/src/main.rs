// chapter4/18_higher_order_functions/src/main.rs
// This is a simple Rust program that demonstrates the use of higher-order functions.
// Higher-order functions are functions that can take other functions as parameters or return them.
// Common examples include `map`, `filter`, and `fold`.

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Using map to transform each element
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squared: {:?}", squared);
    
    // Using filter to keep only elements that satisfy a condition
    let even: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("Even numbers: {:?}", even);
    
    // Using fold to accumulate a result
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);
}
