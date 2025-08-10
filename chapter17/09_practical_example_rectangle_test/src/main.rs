#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_area() {
        let rect = Rectangle::new(5, 10);
        assert_eq!(rect.area(), 50);
    }
    #[test]
    fn test_can_hold_smaller_rectangle() {
        let rect1 = Rectangle::new(5, 10);
        let rect2 = Rectangle::new(3, 6);
        assert!(rect1.can_hold(&rect2));
    }
    #[test]
    fn test_can_hold_larger_rectangle() {
        let rect1 = Rectangle::new(5, 10);
        let rect2 = Rectangle::new(7, 12);
        assert!(!rect1.can_hold(&rect2));
    }
    #[test]
    fn test_can_hold_same_size_rectangle() {
        let rect1 = Rectangle::new(5, 10);
        let rect2 = Rectangle::new(5, 10);
        assert!(!rect1.can_hold(&rect2));
    }
}
 
