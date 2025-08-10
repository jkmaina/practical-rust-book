// LinkedList: doubly-linked list allowing efficient insertion/removal at both ends
// Use when you need frequent insertions/deletions in the middle or at ends
// Less memory efficient than Vec due to pointer overhead, but O(1) insertions anywhere 
use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    println!("Initial list: {:?}", list);
    
    list.push_back(1);
    list.push_back(2);
    println!("After push_back(1, 2): {:?}", list);
    
    list.push_front(0);
    println!("After push_front(0): {:?}", list);
    
    // Remove elements
    let front = list.pop_front();
    println!("Popped from front: {:?}", front);
    println!("List after pop_front: {:?}", list);
    
    let back = list.pop_back();
    println!("Popped from back: {:?}", back);
    println!("Final list: {:?}", list);
}