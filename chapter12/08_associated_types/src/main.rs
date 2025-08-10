// Defining a trait with an associated type
// This is similar to the standard library's Iterator trait
// but simplified for demonstration purposes.

pub trait Iterator {
    type Item;  // Associated type
    
    fn next(&mut self) -> Option<Self::Item>;
    
    // Many other methods with default implementations
}
// Implementing the Iterator trait
struct Counter {
    count: u32,
    max: u32,
}
impl Iterator for Counter {
    type Item = u32;  // Specify the associated type
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
fn main() {
    let mut counter = Counter { count: 0, max: 5 };
    
    while let Some(value) = counter.next() {
        println!("{}", value);
    }
}