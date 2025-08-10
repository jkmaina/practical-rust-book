// String iteration methods: chars() vs bytes() and slicing

fn main() {
    let s = String::from("hello");
    println!("String: {}", s);
    
    // Using string slices (be careful with character boundaries)
    let hello = &s[0..3];
    println!("Slice [0..3]: {}", hello);
    
    // Iterating over characters
    println!("\nCharacters:");
    for c in s.chars() {
        println!("{}", c);
    }
    
    // Iterating over bytes
    println!("\nBytes:");
    for b in s.bytes() {
        println!("{}", b);
    }
    
    // Example with Unicode to show the difference
    let unicode = String::from("नमस्ते");
    println!("\nUnicode string: {}", unicode);
    
    println!("Unicode characters:");
    for c in unicode.chars() {
        println!("{}", c);
    }
    
    println!("Unicode bytes:");
    for b in unicode.bytes() {
        println!("{}", b);
    }
}