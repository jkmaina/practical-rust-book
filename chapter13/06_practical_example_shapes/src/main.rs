use std::f64::consts::PI;
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn description(&self) -> String {
        String::from("A shape")
    }
}
struct Circle {
    radius: f64,
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
    
    fn description(&self) -> String {
        format!("Circle with radius {:.2}", self.radius)
    }
}
struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    
    fn description(&self) -> String {
        format!("Rectangle with width {:.2} and height {:.2}", self.width, self.height)
    }
}
struct Triangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
}
impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Using Heron's formula
        let s = (self.side_a + self.side_b + self.side_c) / 2.0;
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }
    
    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
    
    fn description(&self) -> String {
        format!("Triangle with sides {:.2}, {:.2}, and {:.2}", 
                self.side_a, self.side_b, self.side_c)
    }
}
fn print_shape_info(shape: &impl Shape) {
    println!("{}", shape.description());
    println!("Area: {:.2}", shape.area());
    println!("Perimeter: {:.2}", shape.perimeter());
    println!();
}
fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 4.0, height: 6.0 };
    let triangle = Triangle { side_a: 3.0, side_b: 4.0, side_c: 5.0 };
    
    print_shape_info(&circle);
    print_shape_info(&rectangle);
    print_shape_info(&triangle);
}
