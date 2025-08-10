// This is a simple Rust program demonstrating the use of the loop keyword
// in a loop construct.
// The loop will run indefinitely until a break statement is encountered.
fn main() {
    loop {
        println!("This will print forever!");
        // Without a break, this loop would run indefinitely
        break; // This stops the loop after one iteration
    }
    println!("After the loop");
}
