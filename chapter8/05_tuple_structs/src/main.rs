// This program demonstrates tuple structs in Rust.
// Tuple structs are useful when you want to give a tuple a name and distinguish it from other tuples with the same types.

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0); // Represents an RGB color
    let origin = Point(0, 0, 0); // Represents a point in 3D space

    // Accessing fields of tuple structs by index
    println!("Black color - R: {}, G: {}, B: {}", black.0, black.1, black.2);
    println!("Origin point - x: {}, y: {}, z: {}", origin.0, origin.1, origin.2);
}
