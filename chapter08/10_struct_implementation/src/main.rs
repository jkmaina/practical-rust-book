// This program defines a Rectangle struct and implements methods to calculate its area
// and check if one rectangle can hold another.
// The Rectangle struct has two fields: width and height, both of type u32.
// The methods area and can_hold are implemented for the Rectangle struct.  
// The area method calculates the area of the rectangle by multiplying its width and height.
// The can_hold method checks if the rectangle can hold another rectangle by comparing their dimensions.

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
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

    println!("The area of rect1 is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}