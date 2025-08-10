use std::collections::HashMap;
use std::hash::Hash;
struct ResultCache<K, V, F>
where
    K: Eq + Hash + Clone,
    F: Fn(&K) -> V,
{
    calculation: F,
    cache: HashMap<K, V>,
}
impl<K, V, F> ResultCache<K, V, F>
where
    K: Eq + Hash + Clone,
    F: Fn(&K) -> V,
{
    fn new(calculation: F) -> Self {
        Self {
            calculation,
            cache: HashMap::new(),
        }
    }
    
    fn get(&mut self, key: K) -> &V {
        if !self.cache.contains_key(&key) {
            let value = (self.calculation)(&key);
            self.cache.insert(key.clone(), value);
        }
        
        self.cache.get(&key).unwrap()
    }
    
    fn invalidate(&mut self, key: &K) {
        self.cache.remove(key);
    }
    
    fn clear(&mut self) {
        self.cache.clear();
    }
}
fn main() {
    // Create a cache for an expensive calculation
    let mut fibonacci_cache = ResultCache::new(|&n| {
        println!("Computing fibonacci({})", n);
        match n {
            0 => 0,
            1 => 1,
            n => {
                let mut a = 0;
                let mut b = 1;
                for _ in 2..=n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                }
                b
            }
        }
    });
    
    // First call computes the result
    println!("fibonacci(10) = {}", fibonacci_cache.get(10));
    
    // Second call uses the cached result
    println!("fibonacci(10) = {}", fibonacci_cache.get(10));
    
    // Different input computes a new result
    println!("fibonacci(20) = {}", fibonacci_cache.get(20));
    
    // Clear the cache
    fibonacci_cache.clear();
    
    // This will recompute the result
    println!("fibonacci(10) = {}", fibonacci_cache.get(10));
}