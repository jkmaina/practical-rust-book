// This program checks if a person can enter a venue based on age, ID, and special
// event status using combined logical operators.

fn main() {
    let age = 25;
    let has_id = true;
    let is_special_event = false;
    
    // Can enter if 18+ with ID, or if it's a special event (regardless of age)
    if (age >= 18 && has_id) || is_special_event {
        println!("Welcome to the venue!");
    } else {
        println!("Sorry, you cannot enter.");
    }
}
