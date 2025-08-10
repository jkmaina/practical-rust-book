// VecDeque: double-ended queue allowing efficient insertion/removal at both ends
// Implemented as a growable ring buffer - more memory efficient than LinkedList
// Ideal for queues, stacks, or when you need fast operations at both ends
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();
    println!("Initial queue: {:?}", queue);
    
    // Add to back (enqueue)
    queue.push_back(1);
    queue.push_back(2);
    println!("After push_back(1, 2): {:?}", queue);
    
    // Add to front
    queue.push_front(0);
    println!("After push_front(0): {:?}", queue);
    
    // Remove from front (dequeue)
    let first = queue.pop_front();
    println!("Popped from front: {:?}", first);
    println!("Queue after pop_front: {:?}", queue);
    
    // Remove from back
    let last = queue.pop_back();
    println!("Popped from back: {:?}", last);
    println!("Final queue: {:?}", queue);
    
    // Demonstrate accessing elements without removing
    queue.push_back(10);
    queue.push_back(20);
    println!("\nQueue with new elements: {:?}", queue);
    println!("Front element: {:?}", queue.front());
    println!("Back element: {:?}", queue.back());
}