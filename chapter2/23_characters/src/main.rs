// This program demonstrates the use of characters in Rust, including Unicode characters.
// It defines a few characters and prints them to the console.
// The program includes a regular character, a Unicode character, and an emoji.
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // Unicode for the "double-struck capital Z"
    let heart_eyed_cat = '😻'; // Unicode for the "heart-eyed cat" emoji
    
    println!("Characters: {}, {}, {}", c, z, heart_eyed_cat);
}
