use std::fs::File;
use std::io::{self, Read, Write};
use reqwest;
use thiserror::Error;
#[derive(Error, Debug)]
enum DownloadError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}
fn download_file(url: &str, path: &str) -> Result<(), DownloadError> {
    // Validate URL
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(DownloadError::InvalidUrl(format!("Invalid URL scheme: {}", url)));
    }
    
    // Send HTTP request
    let response = reqwest::blocking::get(url)?;
    
    // Check if the request was successful
    if !response.status().is_success() {
        return Err(DownloadError::InvalidUrl(
            format!("HTTP error: {}", response.status())
        ));
    }    
    // Get the response body
    let body = response.bytes()?;
    
    // Create the file
    let mut file = File::create(path)?;
    
    // Write the body to the file
    file.write_all(&body)?;
    
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let path = "rust-logo.png";
    
    match download_file(url, path) {
        Ok(()) => println!("File downloaded successfully to {}", path),
        Err(e) => eprintln!("Error downloading file: {}", e),
    }
    
    Ok(())
}
