// Memory cache example demonstrating Box<T>, Deref, and Drop traits
// Shows practical use of smart pointers for heap-allocated cached values
// Implements automatic cleanup and memory management

use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug)]
struct Cache<K, V> {
    map: HashMap<K, Box<V>>,
    hits: usize,
    misses: usize,
}

impl<K, V> Cache<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    V: std::fmt::Debug,
{
    fn new() -> Self {
        println!("Creating new cache");
        Cache {
            map: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    
    fn insert(&mut self, key: K, value: V) {
        println!("Caching value: {:?}", value);
        self.map.insert(key, Box::new(value));
    }
    
    fn get(&mut self, key: &K) -> Option<&V> {
        match self.map.get(key) {
            Some(boxed_value) => {
                self.hits += 1;
                println!("Cache hit for key");
                Some(boxed_value.deref())
            },
            None => {
                self.misses += 1;
                println!("Cache miss for key");
                None
            }
        }
    }
    
    fn remove(&mut self, key: &K) -> Option<Box<V>> {
        if let Some(value) = self.map.remove(key) {
            println!("Removed from cache: {:?}", value);
            Some(value)
        } else {
            None
        }
    }
    
    fn size(&self) -> usize {
        self.map.len()
    }
    
    fn stats(&self) -> (usize, usize, f64) {
        let total = self.hits + self.misses;
        let hit_rate = if total > 0 { self.hits as f64 / total as f64 } else { 0.0 };
        (self.hits, self.misses, hit_rate)
    }
    
    fn clear(&mut self) {
        println!("Clearing cache of {} items", self.map.len());
        self.map.clear();
    }
}

impl<K, V> Drop for Cache<K, V> {
    fn drop(&mut self) {
        let (hits, misses, hit_rate) = self.stats();
        println!("Dropping cache - Stats: {} hits, {} misses, {:.2}% hit rate", 
                hits, misses, hit_rate * 100.0);
        println!("Cleaning up {} cached items", self.map.len());
    }
}

// Example of expensive computation that benefits from caching
fn expensive_computation(n: u32) -> String {
    println!("Performing expensive computation for {}", n);
    // Simulate expensive work
    format!("Result for {}", n)
}

fn main() {
    println!("=== Basic Cache Operations ===");
    let mut cache = Cache::new();
    
    // Insert some values
    cache.insert("user:1", "Alice");
    cache.insert("user:2", "Bob");
    cache.insert("config", "production");
    
    println!("Cache size: {}", cache.size());
    
    // Test cache hits and misses
    cache.get(&"user:1");  // Hit
    cache.get(&"user:3");  // Miss
    cache.get(&"user:1");  // Hit again
    
    let (hits, misses, hit_rate) = cache.stats();
    println!("Stats: {} hits, {} misses, {:.1}% hit rate", 
             hits, misses, hit_rate * 100.0);
    
    println!("\n=== Computation Cache Example ===");
    {
        let mut comp_cache: Cache<u32, String> = Cache::new();
        
        // First access - cache miss, computation performed
        let key = 42;
        if comp_cache.get(&key).is_none() {
            let result = expensive_computation(key);
            comp_cache.insert(key, result);
        }
        
        // Second access - cache hit, no computation
        if let Some(cached_result) = comp_cache.get(&key) {
            println!("Cached result: {}", cached_result);
        }
        
        println!("Computation cache will be dropped here");
    } // comp_cache dropped here with stats
    
    println!("\n=== Manual Cleanup ===");
    cache.remove(&"user:2");
    cache.clear();
    
    println!("Main cache will be dropped at end of main");
    // cache dropped here with final stats
}
