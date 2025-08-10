// HashMap ownership behavior: owned vs borrowed values
use std::collections::HashMap;

fn main() {
    // Example 1: Owned values (String) - ownership is moved
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    
    println!("Before insert: {} = {}", field_name, field_value);
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // println!("{}: {}", field_name, field_value); // This would cause a compile error
    
    println!("Map after insert: {:?}", map);
    
    // Example 2: Copy types (i32) - values are copied, originals remain valid
    let key = 1;
    let value = 42;
    let mut map2 = HashMap::new();
    
    println!("\nBefore insert: {} = {}", key, value);
    map2.insert(key, value);
    println!("After insert: {} = {} (still valid)", key, value);
    println!("Map2: {:?}", map2);
    
    // Example 3: Using references to avoid moving ownership
    let name = String::from("Alice");
    let age = String::from("30");
    let mut map3: HashMap<&str, &str> = HashMap::new();
    
    map3.insert(&name, &age);
    println!("\nUsing references - name: {}, age: {}", name, age);
    println!("Map3: {:?}", map3);
}