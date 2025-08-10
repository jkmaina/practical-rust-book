// This program iterates over a collection of animal names and prints each one.

fn main() {
    let animals = ["cat", "dog", "elephant", "fox", "giraffe"];
    
    for animal in animals.iter() {
        println!("The animal is: {}", animal);
    }
}
