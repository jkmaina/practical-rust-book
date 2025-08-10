// This example demonstrates how to read and share data in Rust using a simple Document struct.
// The Document struct allows multiple readers to access its content while ensuring that only one writer can modify it at a time.

struct Document {
    content: String,
    title: String,
}
impl Document {
    fn new(title: &str, content: &str) -> Document {
        Document {
            content: String::from(content),
            title: String::from(title),
        }
    }
    
    fn get_content(&self) -> &str {
        &self.content
    }
    
    fn get_title(&self) -> &str {
        &self.title
    }
    
    fn append_content(&mut self, new_content: &str) {
        self.content.push_str(new_content);
    }
}
fn main() {
    let mut doc = Document::new("My Document", "Hello, world!");
    
    // Multiple readers can access the document
    let title = doc.get_title();
    let content = doc.get_content();
    println!("Title: {}", title);
    println!("Content: {}", content);
    
    // Only one writer can modify the document at a time
    doc.append_content(" Welcome to Rust!");
    
    // We can read again after modification
    println!("Updated content: {}", doc.get_content());
}
