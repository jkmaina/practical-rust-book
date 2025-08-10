// A generic Pair struct with methods to get values, swap them, and find the largest
// value if the type supports comparison

// Define a generic Pair struct
struct Pair<T> {
    first: T,
    second: T,
}
// Implement methods for any type T
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
    
    fn get_first(&self) -> &T {
        &self.first
    }
    
    fn get_second(&self) -> &T {
        &self.second
    }
    
    fn swap(&mut self) {
        std::mem::swap(&mut self.first, &mut self.second);
    }
}
// Implement additional methods for types that can be compared
impl<T: PartialOrd> Pair<T> {
    fn largest(&self) -> &T {
        if self.first >= self.second {
            &self.first
        } else {
            &self.second
        }
    }
}
// Implement Display for Pair if T implements Display
impl<T: std::fmt::Display> std::fmt::Display for Pair<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}
fn main() {
    // Create a pair of integers
    let mut pair = Pair::new(10, 5);
    println!("Original pair: {}", pair);
    println!("Largest value: {}", pair.largest());
    
    // Swap the values
    pair.swap();
    println!("After swap: {}", pair);
    
    // Create a pair of strings
    let string_pair = Pair::new(
        String::from("Hello"),
        String::from("World")
    );
    println!("String pair: {}", string_pair);
}
