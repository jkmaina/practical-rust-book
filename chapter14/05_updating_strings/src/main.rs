// String updating methods: push_str, push, +, and format!

fn main() {
    let mut s = String::from("Hello");
    println!("Initial: {}", s);
    
    // Append a string slice
    s.push_str(", world");
    println!("After push_str: {}", s);
    
    // Append a single character
    s.push('!');
    println!("After push: {}", s);
    
    // Using the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note: s1 is moved here and can no longer be used
    println!("Using + operator: {}", s3);
    // println!("s1: {}", s1); // This would cause an error - s1 is moved
    
    // Using format! macro (doesn't take ownership)
    let s4 = String::from("Hello");
    let s5 = String::from("world");
    let s6 = format!("{}, {}!", s4, s5);
    println!("Using format!: {}", s6);
    println!("s4 still available: {}", s4); // s4 and s5 are still usable
    println!("s5 still available: {}", s5);
}