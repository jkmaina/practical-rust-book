// Iterators: lazy sequences that process elements one at a time
// Two types: iterator adaptors (transform) and consuming adaptors (produce final result)
// More efficient than loops due to zero-cost abstractions

fn main() {
    // Basic iterator usage
    let v = vec![1, 2, 3];
    let mut iter = v.iter();
    println!("Manual iteration:");
    println!("First: {:?}", iter.next());
    println!("Second: {:?}", iter.next());
    println!("Third: {:?}", iter.next());
    println!("Fourth: {:?}", iter.next());
    
    // Iterator adaptors (lazy - don't do work until consumed)
    let v2 = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = v2.iter().map(|x| x * 2).collect();
    println!("\nDoubled: {:?}", doubled);
    
    // Chaining iterator adaptors
    let filtered_doubled: Vec<i32> = v2
        .iter()
        .filter(|&&x| x > 2)
        .map(|x| x * 2)
        .collect();
    println!("Filtered (>2) then doubled: {:?}", filtered_doubled);
    
    // Consuming adaptors
    let sum: i32 = v2.iter().sum();
    println!("Sum: {}", sum);
    
    let max = v2.iter().max();
    println!("Max: {:?}", max);
    
    // for_each (consuming adaptor)
    print!("Elements: ");
    v2.iter().for_each(|x| print!("{} ", x));
    println!();
    
    // Creating iterators from ranges
    let squares: Vec<i32> = (1..6).map(|x| x * x).collect();
    println!("Squares 1-5: {:?}", squares);
}