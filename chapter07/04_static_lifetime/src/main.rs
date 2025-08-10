// This program demonstrates the use and lifecycle of the 'static lifetime in Rust.
// A string literal has a 'static lifetime, meaning it is stored in the program's binary
// and is available for the entire duration of the program.

fn main() {
    // The type &'static str means the string reference is valid for the whole program.
    let s: &'static str = "I have a static lifetime.";

    // The reference 's' can be used anywhere in main, and even passed to other functions
    print_static(s);

    // The string data will remain valid until the program ends
    println!("Back in main: {}", s);
}

// This function takes a reference with a 'static lifetime
fn print_static(text: &'static str) {
    println!("In print_static: {}", text);
}