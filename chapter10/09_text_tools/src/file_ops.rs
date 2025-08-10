// In file_ops.rs
use std::fs;
use std::io;
pub fn read_file(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}
pub fn write_file(filename: &str, content: &str) -> Result<(), io::Error> {
    fs::write(filename, content)
}
 
