// Custom ordering implementation for Person struct
// Demonstrates manual implementation of PartialOrd and Ord traits

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Person {
    name: String,
    age: u32,
}
impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by age, then by name
        self.age.cmp(&other.age)
            .then(self.name.cmp(&other.name))
    }
}
fn main() {
    let people = vec![
        Person { name: String::from("Alice"), age: 35 },
        Person { name: String::from("Bob"), age: 25 },
        Person { name: String::from("Charlie"), age: 30 },
    ];
    
    println!("Original order:");
    for person in &people {
        println!("{:?}", person);
    }
    
    let mut sorted = people.clone();
    sorted.sort();
    
    println!("\nSorted by age, then by name:");
    for person in sorted {
        println!("{:?}", person);
    }
}
