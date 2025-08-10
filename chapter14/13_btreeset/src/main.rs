// BTreeSet: ordered set that maintains elements in sorted order
// Implemented as a B-tree, provides O(log n) operations
// Use when you need sorted unique values or range operations
use std::collections::BTreeSet;

fn main() {
    // BTreeSet automatically maintains sorted order
    let mut numbers = BTreeSet::new();
    numbers.insert(3);
    numbers.insert(1);
    numbers.insert(4);
    numbers.insert(1); // Duplicate, not added
    println!("BTreeSet (auto-sorted): {:?}", numbers);
    
    // Range operations (advantage over HashSet)
    let range: Vec<_> = numbers.range(2..=4).collect();
    println!("Range 2..=4: {:?}", range);
    
    // First and last elements
    println!("First: {:?}", numbers.first());
    println!("Last: {:?}", numbers.last());
    
    // Set operations
    let set1: BTreeSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set2: BTreeSet<i32> = [3, 4, 5].iter().cloned().collect();
    
    println!("\nSet1: {:?}", set1);
    println!("Set2: {:?}", set2);
    println!("Union: {:?}", set1.union(&set2).collect::<Vec<_>>());
    println!("Intersection: {:?}", set1.intersection(&set2).collect::<Vec<_>>());
}