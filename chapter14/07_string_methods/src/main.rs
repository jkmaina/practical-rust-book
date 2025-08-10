// Common string methods for searching, modifying, and checking strings

fn main() {
    let s = String::from("Hello, world!");
    println!("Original string: {}", s);
    
    // Checking if a string contains a substring
    println!("Contains 'world': {}", s.contains("world"));
    
    // Replacing substrings
    let s2 = s.replace("world", "Rust");
    println!("After replace: {}", s2);
    
    // Splitting a string
    println!("Split parts:");
    for part in s.split(", ") {
        println!("  Part: {}", part);
    }
    
    // Trimming whitespace
    let s3 = "   trim me   ".to_string();
    println!("Before trim: '{}'", s3);
    println!("After trim: '{}'", s3.trim());
    
    // Converting case
    println!("Uppercase: {}", s.to_uppercase());
    println!("Lowercase: {}", s.to_lowercase());
    
    // Checking if a string starts or ends with a substring
    println!("Starts with 'Hello': {}", s.starts_with("Hello"));
    println!("Ends with '!': {}", s.ends_with("!"));
}