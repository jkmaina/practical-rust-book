use std::fmt::Display;
struct Pair<T, U> {
    first: T,
    second: U,
}
impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Self { first, second }
    }
}
// This implementation only applies when T implements Display and PartialOrd,
// and U implements Display and Debug
impl<T: Display + PartialOrd, U: Display + std::fmt::Debug> Pair<T, U> {
    fn cmp_display(&self) {
        // Compare T values (both must be same type for comparison)
        println!("First member: {}", self.first);
        println!("Second member: {}", self.second);
        println!("Second member debug: {:?}", self.second);
    }
}

// Additional implementation showing comparison when both types are the same
impl<T: Display + PartialOrd + std::fmt::Debug> Pair<T, T> {
    fn compare_and_display(&self) {
        if self.first >= self.second {
            println!("The largest member is first: {}", self.first);
        } else {
            println!("The largest member is second: {}", self.second);
        }
        println!("Both values: {:?} and {:?}", self.first, self.second);
    }
}

fn main() {
    println!("=== Same types - can use compare_and_display ===");
    let pair1 = Pair::new(10, 5);
    pair1.compare_and_display();  // Available because both are i32
    pair1.cmp_display();          // Also available
    
    println!();
    
    println!("=== Different types - only cmp_display available ===");
    let pair2 = Pair::new(42, "hello");
    pair2.cmp_display();  // Available because both implement Display + Debug
    // pair2.compare_and_display(); // Would NOT compile - different types
    
    println!();
    
    println!("=== Limited functionality - Vec doesn't implement Display ===");
    let pair3 = Pair::new(vec![1, 2, 3], 42);
    // pair3.cmp_display(); // Would NOT compile - Vec<i32> doesn't implement Display
    println!("Can only access basic methods:");
    println!("First length: {}, Second: {}", pair3.first.len(), pair3.second);
}
