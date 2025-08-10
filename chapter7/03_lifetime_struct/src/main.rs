// This program demonstrates how to use lifetime annotations with structs in Rust.
// The struct `ImportantExcerpt` holds a reference to a part of a string.
// The lifetime parameter `'a` ensures that the struct cannot outlive the data it references.

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // Create a String that will live for the duration of main
    let novel = String::from("Call me Ishmael. Some years ago...");
    // Get the first sentence as a string slice (&str)
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // Create an instance of ImportantExcerpt that holds a reference to the first sentence
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // You can use the reference stored in the struct
    println!("Excerpt: {}", i.part);
}
