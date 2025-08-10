use crate::models::{Book, Library};
use crate::io::{get_input, save_library, load_library};
pub fn run_library_system() {
    println!("Library Management System");
    
    let library_name = get_input("Enter library name:");
    let mut library = Library::new(library_name);
    
    // Try to load existing books
    if let Err(e) = load_library(&mut library, "books.csv") {
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
            "1" => add_book(&mut library),
            "2" => remove_book(&mut library),
            "3" => find_book(&library),
            "4" => list_books(&library),
            "5" => {
                if let Err(e) = save_library(&library, "books.csv") {
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
fn add_book(library: &mut Library) {
    let title = get_input("Enter title:");
    let author = get_input("Enter author:");
    let year_str = get_input("Enter year:");
    let year = year_str.parse().unwrap_or(0);
    let isbn = get_input("Enter ISBN:");
    let genre = get_input("Enter genre:");
    
    let book = Book { title, author, year, isbn, genre };
    library.add_book(book);
    println!("Book added successfully!");
}
fn remove_book(library: &mut Library) {
    let isbn = get_input("Enter ISBN of book to remove:");
    match library.remove_book(&isbn) {
        Some(book) => println!("Removed: {} by {}", book.title, book.author),
        None => println!("Book not found!"),
    }
}
fn find_book(library: &Library) {
    let isbn = get_input("Enter ISBN of book to find:");
    match library.get_book(&isbn) {
        Some(book) => println!("Found: {} by {} ({})", book.title, book.author, book.year),
        None => println!("Book not found!"),
    }
}
fn list_books(library: &Library) {
    println!("Books in {}:", library.name);
    for book in library.list_books() {
        println!("{}: {} by {} ({})", book.isbn, book.title, book.author, book.year);
    }
}
