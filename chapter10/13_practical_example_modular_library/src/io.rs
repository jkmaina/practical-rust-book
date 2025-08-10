use std::fs;
use std::io;
use std::path::Path;
use crate::models::{Book, Library};
pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
pub fn save_library(library: &Library, filename: &str) -> io::Result<()> {
    let mut content = String::new();
    
    for (_, book) in library.get_books() {
        content.push_str(&format!("{},{},{},{},{}\n", 
            book.title, book.author, book.year, book.isbn, book.genre));
    }
    
    fs::write(filename, content)
}
pub fn load_library(library: &mut Library, filename: &str) -> io::Result<()> {
    if !Path::new(filename).exists() {
        return Ok(());
    }
    
    let content = fs::read_to_string(filename)?;
    
    for line in content.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 5 {
            let book = Book {
                title: parts[0].to_string(),
                author: parts[1].to_string(),
                year: parts[2].parse().unwrap_or(0),
                isbn: parts[3].to_string(),
                genre: parts[4].to_string(),
            };
            library.add_book(book);
        }
    }
    
    Ok(())
}
