// This program shows how to use a tuple in Rust.
// A tuple is a way to group different types of values together into one compound value.
// Here, we create a tuple with a name, an age, and a boolean for employment status.

fn main() {
    let person = ("Alice", 30, true); //person is defined as a tuple

    // You can access each value in the tuple using a dot and the index (starting from 0).
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is employed: {}", person.2);
}
