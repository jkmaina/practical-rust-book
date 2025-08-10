use std::collections::HashMap;
use std::io;
use std::fs;
use std::path::Path;
// Book struct to represent a book
struct Book {
    title: String,
    author: String,
    year: u32,
    isbn: String,
    genre: String,
}
// Library struct to manage a collection of books
struct Library {
    name: String,
    books: HashMap<String, Book>,
}
impl Library {
    fn new(name: String) -> Library {
        Library {
            name,
            books: HashMap::new(),
        }
    }
    
    fn add_book(&mut self, book: Book) {
        self.books.insert(book.isbn.clone(), book);
    }
    
    fn remove_book(&mut self, isbn: &str) -> Option<Book> {
        self.books.remove(isbn)
    }
    
    fn get_book(&self, isbn: &str) -> Option<&Book> {
        self.books.get(isbn)
    }
    
    fn list_books(&self) {
        println!("Books in {}:", self.name);
        for (isbn, book) in &self.books {
            println!("{}: {} by {} ({})", isbn, book.title, book.author, book.year);
        }
    }
    
    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut content = String::new();
        
        for (_, book) in &self.books {
            content.push_str(&format!("{},{},{},{},{}\n", 
                book.title, book.author, book.year, book.isbn, book.genre));
        }
        
        fs::write(filename, content)
    }
    
    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
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
                self.add_book(book);
            }
        }
        
        Ok(())
    }
}
// Function to get user input
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
fn main() {
    println!("Library Management System");
    
    let library_name = get_input("Enter library name:");
    let mut library = Library::new(library_name);
    
    // Try to load existing books
    if let Err(e) = library.load_from_file("books.csv") {
        println!("Error loading books: {}", e);
    }
    
    loop {
        println!("\nOptions:");
        println!("1. Add a book");
        println!("2. Remove a book");
        println!("3. Find a book");
        println!("4. List all books");
        println!("5. Save and exit");
        
        let choice = get_input("Enter your choice (1-5):");
        
        match choice.as_str() {
            "1" => {
                let title = get_input("Enter title:");
                let author = get_input("Enter author:");
                let year_str = get_input("Enter year:");
                let year = year_str.parse().unwrap_or(0);
                let isbn = get_input("Enter ISBN:");
                let genre = get_input("Enter genre:");
                
                let book = Book { title, author, year, isbn, genre };
                library.add_book(book);
                println!("Book added successfully!");
            },
            "2" => {
                let isbn = get_input("Enter ISBN of book to remove:");
                match library.remove_book(&isbn) {
                    Some(book) => println!("Removed: {} by {}", book.title, book.author),
                    None => println!("Book not found!"),
                }
            },
            "3" => {
                let isbn = get_input("Enter ISBN of book to find:");
                match library.get_book(&isbn) {
                    Some(book) => println!("Found: {} by {} ({})", book.title, book.author, book.year),
                    None => println!("Book not found!"),
                }
            },
            "4" => {
                library.list_books();
            },
            "5" => {
                if let Err(e) = library.save_to_file("books.csv") {
                    println!("Error saving books: {}", e);
                } else {
                    println!("Books saved successfully!");
                }
                break;
            },
            _ => println!("Invalid choice!"),
        }
    }
    
    println!("Goodbye!");
}
