use std::io;
fn main() {
    println!("What's your name?");
    
    let mut name = String::new();
    
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    // Trim the newline character
    let name = name.trim();
    
    println!("Hello, {}! Nice to meet you!", name);
}