// Basic Option<T> example demonstrating Some and None variants
// Shows how to create, print, and pattern match Option values

fn main() {
    // Creating Option values
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // Printing Option values
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    // Using if let to extract value
    if let Some(value) = some_number {
        println!("Number is: {}", value);
    }
    // Using match to handle both cases
    match absent_number {
        Some(value) => println!("Got value: {}", value),
        None => println!("No value present"),
    }
}