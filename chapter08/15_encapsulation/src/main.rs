// This code demonstrates encapsulation in Rust by defining a Rectangle struct
// with private fields and public methods to interact with those fields.
// The area of the rectangle can be calculated using a public method.

pub struct Rectangle {
    width: u32,
    height: u32,
} 

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::new(10, 5);
    println!("The area of the rectangle is: {}", rect.area());
}