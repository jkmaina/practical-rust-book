// Command pattern example with undo functionality
use std::rc::Rc;
use std::cell::RefCell;

// Command trait
trait Command {
    fn execute(&self) -> Result<(), String>;
    fn undo(&self) -> Result<(), String>;
    fn name(&self) -> &str;
}

// A simple text document
struct Document {
    content: String,
}
impl Document {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
    
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    
    fn delete_text(&mut self, count: usize) {
        let new_len = self.content.len().saturating_sub(count);
        self.content.truncate(new_len);
    }
    
    fn content(&self) -> &str {
        &self.content
    }
}
// AddTextCommand
struct AddTextCommand {
    document: Rc<RefCell<Document>>,
    text: String,
}

impl AddTextCommand {
    fn new(document: Rc<RefCell<Document>>, text: &str) -> Self {
        Self {
            document,
            text: text.to_string(),
        }
    }
}

impl Command for AddTextCommand {
    fn execute(&self) -> Result<(), String> {
        self.document.borrow_mut().add_text(&self.text);
        Ok(())
    }
    
    fn undo(&self) -> Result<(), String> {
        self.document.borrow_mut().delete_text(self.text.len());
        Ok(())
    }
    
    fn name(&self) -> &str {
        "Add Text"
    }
}
// DeleteTextCommand
struct DeleteTextCommand {
    document: Rc<RefCell<Document>>,
    count: usize,
    deleted_text: String,
}

impl DeleteTextCommand {
    fn new(document: Rc<RefCell<Document>>, count: usize) -> Self {
        let doc_borrow = document.borrow();
        let start = doc_borrow.content().len().saturating_sub(count);
        let deleted_text = doc_borrow.content()[start..].to_string();
        drop(doc_borrow);
        
        Self {
            document,
            count,
            deleted_text,
        }
    }
}

impl Command for DeleteTextCommand {
    fn execute(&self) -> Result<(), String> {
        self.document.borrow_mut().delete_text(self.count);
        Ok(())
    }
    
    fn undo(&self) -> Result<(), String> {
        self.document.borrow_mut().add_text(&self.deleted_text);
        Ok(())
    }
    
    fn name(&self) -> &str {
        "Delete Text"
    }
}
// CommandHistory to track and undo commands
struct CommandHistory {
    commands: Vec<Box<dyn Command>>,
}
impl CommandHistory {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
    
    fn execute(&mut self, command: Box<dyn Command>) -> Result<(), String> {
        println!("Executing: {}", command.name());
        command.execute()?;
        self.commands.push(command);
        Ok(())
    }
    
    fn undo_last(&mut self) -> Result<(), String> {
        if let Some(command) = self.commands.pop() {
            println!("Undoing: {}", command.name());
            command.undo()?;
        }
        Ok(())
    }
}
fn main() {
    let document = Rc::new(RefCell::new(Document::new()));
    let mut history = CommandHistory::new();
    
    // Execute commands
    history.execute(Box::new(AddTextCommand::new(document.clone(), "Hello, "))).unwrap();
    println!("Document: '{}'", document.borrow().content());
    
    history.execute(Box::new(AddTextCommand::new(document.clone(), "world!"))).unwrap();
    println!("Document: '{}'", document.borrow().content());
    
    history.execute(Box::new(DeleteTextCommand::new(document.clone(), 6))).unwrap();
    println!("Document: '{}'", document.borrow().content());
    
    // Undo commands
    history.undo_last().unwrap();
    println!("After undo: '{}'", document.borrow().content());
    
    history.undo_last().unwrap();
    println!("After undo: '{}'", document.borrow().content());
    
    history.undo_last().unwrap();
    println!("After undo: '{}'", document.borrow().content());
}
