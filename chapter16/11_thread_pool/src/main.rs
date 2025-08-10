// Thread pool implementation demonstrating advanced concurrency patterns
// Shows Arc<Mutex<T>> for shared mutable state across threads
// Combines channels, mutexes, and thread management for efficient task execution

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// ThreadPool manages a fixed number of worker threads
pub struct ThreadPool {
    workers: Vec<Worker>,           // Collection of worker threads
    sender: mpsc::Sender<Job>,      // Channel to send jobs to workers
}

// Job is a boxed closure that can be executed by any worker thread
// Must be Send (thread-safe) and 'static (no borrowed references)
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool with specified number of worker threads
    ///
    /// # Arguments
    /// * `size` - Number of worker threads to create
    ///
    /// # Panics
    /// Panics if size is zero (need at least one worker)
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "ThreadPool size must be greater than zero");
        
        println!("Creating ThreadPool with {} workers", size);
        
        // Create channel for sending jobs to workers
        let (sender, receiver) = mpsc::channel();
        
        // Wrap receiver in Arc<Mutex<T>> to share among multiple worker threads
        // Arc: allows multiple owners (workers can share the receiver)
        // Mutex: ensures only one worker can receive a job at a time
        let receiver = Arc::new(Mutex::new(receiver));
        
        // Pre-allocate vector for workers
        let mut workers = Vec::with_capacity(size);
        
        // Create worker threads
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender }
    }
    
    /// Execute a closure on one of the worker threads
    /// The closure is queued and will be executed when a worker becomes available
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Box the closure to create a Job
        let job = Box::new(f);
        
        // Send job through channel - any available worker will pick it up
        self.sender.send(job).unwrap();
    }
    
    /// Get the number of workers in the pool
    pub fn size(&self) -> usize {
        self.workers.len()
    }
}

// Worker represents a single thread that executes jobs
#[allow(dead_code)]
struct Worker {
    id: usize,                      // Worker identifier
    thread: thread::JoinHandle<()>, // Handle to the worker thread
}

impl Worker {
    /// Create a new worker thread that listens for jobs
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        println!("Creating worker {}", id);
        
        // Spawn a thread that continuously waits for jobs
        let thread = thread::spawn(move || {
            loop {
                // Lock the mutex to access the receiver
                // Only one worker can receive a job at a time
                let job = match receiver.lock() {
                    Ok(receiver) => {
                        match receiver.recv() {
                            Ok(job) => job,
                            Err(_) => {
                                // Sender has been dropped, no more jobs coming
                                println!("Worker {} shutting down - no more jobs", id);
                                break;
                            }
                        }
                    }
                    Err(_) => {
                        // Mutex is poisoned, another thread panicked
                        println!("Worker {} shutting down - mutex poisoned", id);
                        break;
                    }
                };
                
                println!("Worker {} got a job; executing.", id);
                
                // Execute the job (mutex is automatically released here)
                job();
                
                println!("Worker {} finished job.", id);
            }
        });
        
        Worker { id, thread }
    }
}

fn main() {
    println!("=== Thread Pool Example ===");
    
    // Create a thread pool with 4 worker threads
    let pool = ThreadPool::new(4);
    
    println!("\nSubmitting 8 jobs to the thread pool...");
    
    // Submit multiple jobs to the thread pool
    for i in 0..8 {
        pool.execute(move || {
            println!("Job {} starting on thread {:?}", i, thread::current().id());
            
            // Simulate work with different durations
            let work_duration = Duration::from_millis(500 + (i * 200) % 1000);
            thread::sleep(work_duration);
            
            println!("Job {} completed after {:?}", i, work_duration);
        });
    }
    
    println!("All jobs submitted. Workers will process them concurrently.");
    
    // Give workers time to complete all jobs
    thread::sleep(Duration::from_secs(3));
    
    // Drop the pool to signal workers to shut down
    drop(pool);
    println!("\nThreadPool dropped - workers will shut down gracefully");
    
    println!("\n=== Thread Pool Benefits ===");
    println!("1. Fixed number of threads - no overhead of creating/destroying threads");
    println!("2. Work queue - jobs are distributed among available workers");
    println!("3. Load balancing - idle workers pick up new jobs automatically");
    println!("4. Resource control - limits concurrent thread usage");
    println!("5. Efficient for many small tasks");
    
    println!("\n=== Key Concepts Demonstrated ===");
    println!("- Arc<Mutex<T>>: Shared ownership of mutable data across threads");
    println!("- mpsc::channel: Communication between main thread and workers");
    println!("- Box<dyn Trait>: Type erasure for storing different closure types");
    println!("- Thread spawning and management");
    println!("- Concurrent job execution with load balancing");
}
