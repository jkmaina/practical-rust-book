// HashSet: unordered set with fast O(1) average-case operations
// Implemented using hash table, provides fastest lookups and insertions
// Use when you need unique values and don't care about order
use std::collections::HashSet;

fn main() {
    // HashSet stores unique values in arbitrary order
    let mut fruit = HashSet::new();
    fruit.insert("apple");
    fruit.insert("banana");
    fruit.insert("apple"); // Duplicate, not added
    println!("HashSet (unordered): {:?}", fruit);
    
    // Fast membership testing - main advantage
    println!("Contains apple: {}", fruit.contains("apple"));
    println!("Contains orange: {}", fruit.contains("orange"));
    
    // Set operations
    let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
    
    println!("\nSet1: {:?}", set1);
    println!("Set2: {:?}", set2);
    println!("Union: {:?}", set1.union(&set2).collect::<Vec<_>>());
    println!("Intersection: {:?}", set1.intersection(&set2).collect::<Vec<_>>());
    println!("Difference: {:?}", set1.difference(&set2).collect::<Vec<_>>());
    
    // Creating from iterator
    let numbers: HashSet<i32> = (1..=5).collect();
    println!("\nFrom range: {:?}", numbers);
}