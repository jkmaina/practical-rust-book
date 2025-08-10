// Basic HashMap operations: creation, insertion, and access
use std::collections::HashMap;

fn main() {
    // Creating an empty hash map
    let mut scores = HashMap::new();
    
    // Inserting values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores: {:?}", scores);
    
    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("Blue team score: {}", s),
        None => println!("Blue team not found"),
    }
    
    // Creating from vectors of keys and values
    let teams = vec![String::from("Red"), String::from("Green")];
    let initial_scores = vec![25, 75];
    let scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("Scores2: {:?}", scores2);
    
    // Iterating over key-value pairs
    println!("All scores:");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}