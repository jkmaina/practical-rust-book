// This is a simple Rust program that demonstrates ownership and borrowing.
// It defines a function that takes ownership of a string and returns its length.
// The main function shows how ownership works in Rust, particularly with strings.
// If you try to use the string after passing it to the function, it will result in a compile-time error.

fn main() {
    let mut text = String::from("Hello");
    
    // First, we'll append to the text
    append_greeting(&mut text);
    println!("After appending: {}", text);
    
    // Then, we'll calculate some statistics
    let stats = calculate_stats(&text);
    println!("Text length: {}", stats.0);
    println!("Word count: {}", stats.1);
    
    // Finally, we'll convert to uppercase
    to_uppercase(&mut text);
    println!("Uppercase: {}", text);
}
fn append_greeting(text: &mut String) {
    text.push_str(", world!");
}
fn calculate_stats(text: &String) -> (usize, usize) {
    let length = text.len();
    let word_count = text.split_whitespace().count();
    (length, word_count)
}
fn to_uppercase(text: &mut String) {
    *text = text.to_uppercase();
}

