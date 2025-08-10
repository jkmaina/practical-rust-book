// This program demonstrates the use of boolean operations in Rust.
// It checks conditions using logical AND, OR, and NOT operations.
// The program determines whether to go to the park, take a day off, or if it is a working day. 
fn main() {
    let is_weekend = true;
    let is_sunny = true;
    
    // Logical AND (&&)
    let go_to_park = is_weekend && is_sunny;
    println!("Should I go to the park? {}", go_to_park);
    
    // Logical OR (||)
    let is_holiday = false;
    let take_day_off = is_weekend || is_holiday;
    println!("Should I take the day off? {}", take_day_off);
    
    // Logical NOT (!)
    let is_working_day = !is_weekend;
    println!("Is it a working day? {}", is_working_day);
}
