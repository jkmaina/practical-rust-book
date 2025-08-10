// Custom error types example showing how to create and handle application-specific errors
// Demonstrates error conversion with From trait and ? operator usage

use std::fmt;
use std::error::Error;
use std::fs;

#[derive(Debug)]
enum AppError {
    FileError(std::io::Error),
    ParseError(std::num::ParseIntError),
    InvalidInput(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::FileError(err) => write!(f, "File error: {}", err),
            AppError::ParseError(err) => write!(f, "Parse error: {}", err),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::FileError(err)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> AppError {
        AppError::ParseError(err)
    }
}

fn read_number_from_file(filename: &str) -> Result<i32, AppError> {
    let content = fs::read_to_string(filename)?;  // Converts io::Error to AppError
    let trimmed = content.trim();
    
    if trimmed.is_empty() {
        return Err(AppError::InvalidInput("File is empty".to_string()));
    }
    
    let number = trimmed.parse::<i32>()?;  // Converts ParseIntError to AppError
    Ok(number)
}

fn main() {
    match read_number_from_file("number.txt") {
        Ok(number) => println!("Number from file: {}", number),
        Err(error) => println!("Error: {}", error),
    }
}