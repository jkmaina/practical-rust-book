// Fuzz testing example using cargo-fuzz and libfuzzer
// Generates random inputs to find crashes, panics, or unexpected behavior
// Tests the add function with arbitrary byte sequences converted to integers
// REQUIRES NIGHTLY RUST: rustup default nightly
// Run with: cargo fuzz run fuzz_add

#![no_main]
use libfuzzer_sys::fuzz_target;
use cargo_fuzz_example::add;

fuzz_target!(|data: &[u8]| {
    // Need at least 8 bytes to create two i32 values
    if data.len() >= 8 {
        // Convert first 4 bytes to i32
        let a = i32::from_ne_bytes([data[0], data[1], data[2], data[3]]);
        // Convert next 4 bytes to i32
        let b = i32::from_ne_bytes([data[4], data[5], data[6], data[7]]);
        
        // Test the add function - fuzzer will find inputs that cause panics
        let result = add(a, b);
        
        // Optional: Add property checks
        // Verify commutativity: a + b == b + a
        assert_eq!(result, add(b, a));
        
        // Verify that result is within expected bounds (catch overflow)
        if let Some(expected) = a.checked_add(b) {
            assert_eq!(result, expected);
        }
    }
});
 
