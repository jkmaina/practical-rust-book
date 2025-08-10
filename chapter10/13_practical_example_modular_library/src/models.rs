use std::collections::HashMap;
#[derive(Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
    pub isbn: String,
    pub genre: String,
}
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
    
    pub fn add_book(&mut self, book: Book) {
        self.books.insert(book.isbn.clone(), book);
    }
    
    pub fn remove_book(&mut self, isbn: &str) -> Option<Book> {
        self.books.remove(isbn)
    }
    
    pub fn get_book(&self, isbn: &str) -> Option<&Book> {
        self.books.get(isbn)
    }
    
    pub fn list_books(&self) -> Vec<&Book> {
        self.books.values().collect()
    }
    
    pub fn get_books(&self) -> &HashMap<String, Book> {
        &self.books
    }
}
