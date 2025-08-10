// This program demonstrates how to use structs as function parameters in Rust.
// The Rectangle struct is defined, and the area function takes a reference to a Rectangle.

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "The area of rect1 is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The area of rect2 is {} square pixels.",
        area(&rect2)
    );
    println!(
        "The area of rect3 is {} square pixels.",
        area(&rect3)
    );
}

// This function takes a reference to a Rectangle and returns its area.
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
