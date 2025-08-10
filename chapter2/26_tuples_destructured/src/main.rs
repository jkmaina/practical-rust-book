// This program shows how to use a tuple returned from a function in Rust.
// It defines a function that returns a tuple containing a name, age, and employment status.
// The main function calls this function and prints the values of the tuple.

fn get_person_info() -> (&'static str, i32, bool) {
    // Return a tuple with name, age, and employment status
    ("Charlie", 35, true)
}

fn main() {
    // Get the tuple and split it into separate variables
    let (name, age, is_employed) = get_person_info();

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is employed: {}", is_employed);
}
