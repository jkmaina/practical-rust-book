// This example demonstrates how to use getters and setters in Rust with a Rectangle struct.
// Getters allow you to access private fields, while setters allow you to modify them.
// This encapsulation helps maintain control over how the data is accessed and modified.    

pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Constructor: creates a new Rectangle with the given width and height
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Getter for width
    pub fn width(&self) -> u32 {
        self.width
    }

    // Getter for height
    pub fn height(&self) -> u32 {
        self.height
    }

    // Setter for width
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // Setter for height
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

fn main() {
    // Create a new Rectangle
    let mut rect = Rectangle::new(30, 50);

    // Print initial dimensions
    println!("Initial width: {}", rect.width());
    println!("Initial height: {}", rect.height());

    // Change the dimensions using setters
    rect.set_width(60);
    rect.set_height(80);

    // Print updated dimensions
    println!("Updated width: {}", rect.width());
    println!("Updated height: {}", rect.height());
}
