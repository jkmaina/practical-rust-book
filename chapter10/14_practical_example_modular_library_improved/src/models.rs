use std::collections::HashMap;

/// Represents a book with title, author, year, ISBN, and genre.
#[derive(Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
    pub isbn: String,
    pub genre: String,
}
/// Represents a library with a name and a collection of books.
pub struct Library {
    pub name: String,
    books: HashMap<String, Book>,
}
impl Library {
    pub fn new(name: String) -> Library {
        Library {
            name,
            books: HashMap::new(),
        }
    }
        /// Adds a book to the library.
    pub fn add_book(&mut self, book: Book) {
        self.books.insert(book.isbn.clone(), book);
    }
    /// Removes a book from the library by ISBN.
    pub fn remove_book(&mut self, isbn: &str) -> Option<Book> {
        self.books.remove(isbn)
    }

    /// Gets a reference to a book by ISBN.
    pub fn get_book(&self, isbn: &str) -> Option<&Book> {
        self.books.get(isbn)
    }

    /// Lists all books in the library. 
    pub fn list_books(&self) -> Vec<&Book> {
        self.books.values().collect()
    }
    
    /// Returns a reference to the books HashMap.
    pub fn get_books(&self) -> &HashMap<String, Book> {
        &self.books
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_book() {
        let mut library = Library::new("Test Library".to_string());
        let book = Book {
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            year: 2023,
            isbn: "1234567890".to_string(),
            genre: "Test".to_string(),
        };
        
        library.add_book(book.clone());
        
        assert_eq!(library.get_book(&book.isbn).unwrap().title, book.title);
    }
    
    #[test]
    fn test_remove_book() {
        let mut library = Library::new("Test Library".to_string());
        let book = Book {
            title: "Test Book".to_string(),
            author: "Test Author".to_string(),
            year: 2023,
            isbn: "1234567890".to_string(),
            genre: "Test".to_string(),
        };
        
        library.add_book(book.clone());
        let removed_book = library.remove_book(&book.isbn).unwrap();
        
        assert_eq!(removed_book.title, book.title);
        assert!(library.get_book(&book.isbn).is_none());
    }
}
