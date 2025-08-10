// Basic String creation methods and conversions

fn main() {
    // Creating an empty String
    let mut s1 = String::new();
    s1.push_str("added text");
    println!("s1: {}", s1);
    
    // Creating a String from a string literal
    let s2 = String::from("hello");
    println!("s2: {}", s2);
    
    let s3 = "hello".to_string();
    println!("s3: {}", s3);
    
    // Creating a String with capacity
    let mut s4 = String::with_capacity(10);
    println!("s4 capacity: {}", s4.capacity());
    s4.push_str("test");
    println!("s4: {}", s4);
    
    // Creating a String from other types
    let s5 = 42.to_string();
    println!("s5: {}", s5);
    
    let s6 = true.to_string();
    println!("s6: {}", s6);
}