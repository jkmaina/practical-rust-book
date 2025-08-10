fn five() { // No return type specified
    return 5; // This value is returned implicitly, but the type isn't declared
}

fn main() {
    let x = five();
    println!("{}", x);
}