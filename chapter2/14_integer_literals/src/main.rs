fn main() {
    let decimal = 98_222;      // Decimal (base 10)
    let hex = 0xff;            // Hexadecimal (base 16)
    let octal = 0o77;          // Octal (base 8)
    let binary = 0b1111_0000;  // Binary (base 2)
    let byte = b'A';           // Byte (u8 only, represents ASCII character)
    
    println!("Decimal: {}", decimal);
    println!("Hexadecimal: {}", hex);
    println!("Octal: {}", octal);
    println!("Binary: {}", binary);
    println!("Byte: {}", byte);
}
