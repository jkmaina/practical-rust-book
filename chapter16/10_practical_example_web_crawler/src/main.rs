// Practical web crawler example demonstrating concurrent HTTP requests
// Uses threads and channels to fetch multiple URLs simultaneously
// Shows real-world application of Rust's concurrency primitives

use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn main() {
    println!("=== Concurrent Web Crawler ===");
    
    // List of URLs to fetch concurrently
    let urls = vec![
        "https://www.rust-lang.org",
        "https://doc.rust-lang.org",
        "https://crates.io",
        "https://blog.rust-lang.org",
    ];
    
    println!("Fetching {} URLs concurrently...", urls.len());
    
    // Create a channel for sending results back to main thread
    // Results are tuples: (url, content_length, optional_error)
    let (tx, rx) = mpsc::channel();
    
    // Start timing the concurrent operations
    let start = Instant::now();
    
    // Spawn a separate thread for each URL
    for url in urls {
        // Clone the transmitter for each thread
        // Each thread needs its own transmitter to send results
        let tx = tx.clone();
        
        // Spawn thread with move to transfer ownership of url and tx
        thread::spawn(move || {
            println!("Thread started for: {}", url);
            
            // Attempt to fetch the URL
            match fetch_url(url) {
                Ok(content) => {
                    let len = content.len();
                    println!("Successfully fetched {}: {} bytes", url, len);
                    // Send success result: (url, length, no_error)
                    tx.send((url, len, None)).unwrap();
                }
                Err(err) => {
                    println!("Failed to fetch {}: {}", url, err);
                    // Send error result: (url, 0_bytes, error)
                    tx.send((url, 0, Some(err.to_string()))).unwrap();
                }
            }
        });
    }
    
    // Drop the original transmitter so the receiver knows when all threads are done
    // Without this, the receiver would wait forever
    drop(tx);
    
    println!("All threads spawned, waiting for results...");
    
    // Collect and print results as they arrive
    // The loop ends when all transmitters are dropped (all threads finished)
    for (url, len, error) in rx {
        match error {
            None => println!("✓ {}: {} bytes", url, len),
            Some(err) => println!("✗ {}: Error: {}", url, err),
        }
    }
    
    // Calculate and display total execution time
    let duration = start.elapsed();
    println!("\nAll requests completed in: {:?}", duration);
    
    println!("\n=== Concurrency Benefits ===");
    println!("1. Multiple HTTP requests happen simultaneously");
    println!("2. Total time ≈ slowest request (not sum of all requests)");
    println!("3. Threads handle I/O blocking independently");
    println!("4. Results arrive as soon as each request completes");
}

// Function to fetch content from a URL
// Returns Result to handle network errors gracefully
fn fetch_url(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    println!("Fetching: {}", url);
    
    // Simulate HTTP request (replace with actual HTTP client in real usage)
    // Using thread::sleep to simulate network delay
    use std::time::Duration;
    thread::sleep(Duration::from_millis(500 + (url.len() % 1000) as u64));
    
    // Simulate different outcomes based on URL
    if url.contains("crates") {
        // Simulate an error for demonstration
        Err("Simulated network error".into())
    } else {
        // Simulate successful response
        let content = format!("<html><body>Content from {}</body></html>", url);
        Ok(content)
    }
}
