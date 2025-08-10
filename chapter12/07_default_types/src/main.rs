// Demonstrates default type parameters in a generic struct
// This is useful for simplifying type annotations and providing sensible defaults
// when the user does not specify a type.
// Here, we define a Vector struct with a default type of f64.
// We also implement a method to add two vectors together.

use std::ops::Add;
struct Vector<T = f64> {
    x: T,
    y: T,
}
impl<T> Vector<T>
where
    T: Add<Output = T> + Copy,
{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    
    fn add(&self, other: &Vector<T>) -> Vector<T> {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn main() {
    // Uses the default f64 type
    let v1 = Vector { x: 1.0, y: 2.0 };
    let v2 = Vector { x: 3.0, y: 4.0 };
    let v3 = v1.add(&v2);
    println!("f64 vectors: ({}, {}) + ({}, {}) = ({}, {})", 
             v1.x, v1.y, v2.x, v2.y, v3.x, v3.y);
    
    // Explicitly specifies i32
    let v4: Vector<i32> = Vector::new(1, 2);
    let v5: Vector<i32> = Vector::new(3, 4);
    let v6 = v4.add(&v5);
    println!("i32 vectors: ({}, {}) + ({}, {}) = ({}, {})", 
             v4.x, v4.y, v5.x, v5.y, v6.x, v6.y);
    
    // Default type inference
    let v7 = Vector::new(1.5, 2.5);  // Inferred as f64
    println!("Default inferred: ({}, {})", v7.x, v7.y);
}