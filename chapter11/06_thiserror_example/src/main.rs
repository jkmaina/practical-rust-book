// Using thiserror crate to simplify custom error handling
// Automatically generates Display and From implementations

use thiserror::Error;
use std::fs;

#[derive(Error, Debug)]
enum AppError {
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

fn read_number_from_file(filename: &str) -> Result<i32, AppError> {
    let content = fs::read_to_string(filename)?;  // Auto-converts io::Error
    let trimmed = content.trim();
    
    if trimmed.is_empty() {
        return Err(AppError::InvalidInput("File is empty".to_string()));
    }
    
    let number = trimmed.parse::<i32>()?;  // Auto-converts ParseIntError
    Ok(number)
}

fn main() {
    match read_number_from_file("number.txt") {
        Ok(number) => println!("Number from file: {}", number),
        Err(error) => println!("Error: {}", error),
    }
}