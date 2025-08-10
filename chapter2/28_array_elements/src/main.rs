fn main() {
    let fruits: [&str; 5] = ["apple", "banana", "cherry", "date", "elderberry"];
    
    let first = fruits[0];
    let second = fruits[1];
    
    println!("First fruit: {}", first);
    println!("Second fruit: {}", second);
}
