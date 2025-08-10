pub trait Measurable {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
pub struct Circle {
    pub radius: f64,
}
impl Measurable for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 5.0 };
        assert_eq!(circle.area(), std::f64::consts::PI * 25.0);
    }
    #[test]
    fn test_circle_perimeter() {
        let circle = Circle { radius: 5.0 };
        assert_eq!(circle.perimeter(), 2.0 * std::f64::consts::PI * 5.0);
    }
} 
