#[derive(Debug, PartialEq, Eq)]
struct Id {
    value: i32,
}
#[derive(Debug, PartialEq)]
struct Temperature {
    degrees: f64,
}
fn main() {
    let id1 = Id { value: 1 };
    let id2 = Id { value: 1 };
    let id3 = Id { value: 2 };
    
    println!("id1 == id2: {}", id1 == id2);  // true
    println!("id1 == id3: {}", id1 == id3);  // false
    
    let temp1 = Temperature { degrees: 36.6 };
    let temp2 = Temperature { degrees: 36.6 };
    
    println!("temp1 == temp2: {}", temp1 == temp2);  // true
    
    // NaN example
    let nan = f64::NAN;
    println!("NaN == NaN: {}", nan == nan);  // false, demonstrating partial equivalence
}
