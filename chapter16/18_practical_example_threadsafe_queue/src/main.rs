use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
struct ThreadSafeQueue<T> {
    queue: Mutex<Vec<T>>,
    not_empty: Condvar,
}
impl<T> ThreadSafeQueue<T> {
    fn new() -> Self {
        ThreadSafeQueue {
            queue: Mutex::new(Vec::new()),
            not_empty: Condvar::new(),
        }
    }
    
    fn enqueue(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(item);
        self.not_empty.notify_one();
    }
    
    fn dequeue(&self) -> T {
        let mut queue = self.queue.lock().unwrap();
        while queue.is_empty() {
            queue = self.not_empty.wait(queue).unwrap();
        }
        queue.remove(0)
    }
    
    fn is_empty(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.is_empty()
    }
    
    fn len(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }
}
fn main() {
    // Create a thread-safe queue
    let queue = Arc::new(ThreadSafeQueue::new());
    
    // Clone the Arc for the producer thread
    let producer_queue = Arc::clone(&queue);
    
    // Spawn a producer thread
    let producer = thread::spawn(move || {
        for i in 0..5 {
            println!("Producing: {}", i);
            producer_queue.enqueue(i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // Clone the Arc for the consumer thread
    let consumer_queue = Arc::clone(&queue);
    
    // Spawn a consumer thread
    let consumer = thread::spawn(move || {
        for _ in 0..5 {
            let item = consumer_queue.dequeue();
            println!("Consuming: {}", item);
            thread::sleep(Duration::from_millis(200));
        }
    });
    
    // Wait for both threads to finish
    producer.join().unwrap();
    consumer.join().unwrap();
}
 
