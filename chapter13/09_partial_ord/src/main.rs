// PartialOrd and Ord traits enable comparison operations
// PartialOrd: <, >, <=, >= operators (partial ordering)
// Ord: extends PartialOrd for total ordering (all values comparable)

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

fn main() {
    let v1 = Version { major: 1, minor: 0, patch: 0 };
    let v2 = Version { major: 1, minor: 2, patch: 3 };
    let v3 = Version { major: 2, minor: 0, patch: 0 };
    
    println!("v1 < v2: {}", v1 < v2);  // true
    println!("v2 < v3: {}", v2 < v3);  // true
    
    let versions = vec![v3, v1, v2];
    let mut sorted = versions.clone();
    sorted.sort();
    
    println!("Original: {:?}", versions);
    println!("Sorted: {:?}", sorted);
}
 
