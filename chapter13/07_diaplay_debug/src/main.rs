use std::fmt::{Display, Formatter, Result};
struct Point {
    x: i32,
    y: i32,
}
// Implementing Debug automatically with a derive attribute
#[derive(Debug)]
struct ColoredPoint {
    x: i32,
    y: i32,
    color: String,
}
// Implementing Display manually
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = ColoredPoint { x: 3, y: 4, color: String::from("blue") };
    
    // Using Display
    println!("p1: {}", p1);
    
    // Using Debug
    println!("p2: {:?}", p2);
    
    // Using pretty-print Debug
    println!("p2 (pretty): {:#?}", p2);
}
