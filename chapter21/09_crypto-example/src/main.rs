use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
fn main() {
    // Generate a random key
    let key = Aes256Gcm::generate_key(OsRng);
    
    // Create a cipher instance
    let cipher = Aes256Gcm::new(&key);
    
    // Generate a random nonce
    let nonce = Aes256Gcm::generate_nonce(OsRng); // 96-bits; unique per message
    
    // Encrypt the message
    let message = b"Hello, Rust Cryptography!";
    let ciphertext = cipher
        .encrypt(&nonce, message.as_ref())
        .expect("Encryption failed");
    
    println!("Original message: {:?}", std::str::from_utf8(message).unwrap());
    println!("Encrypted: {:?}", ciphertext);
    
    // Decrypt the message
    let plaintext = cipher
        .decrypt(&nonce, ciphertext.as_ref())
        .expect("Decryption failed");
    
    println!("Decrypted: {:?}", std::str::from_utf8(&plaintext).unwrap());
}
 
