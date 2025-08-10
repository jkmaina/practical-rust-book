enum EditorState {
    Normal,
    Insert,
    Visual,
    Command,
}
enum EditorCommand {
    MoveCursor(Direction, usize),
    InsertText(String),
    DeleteText(usize),
    ChangeMode(EditorState),
    Save,
    Quit,
}
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Editor {
    state: EditorState,
    content: String,
    cursor_position: usize,
}
impl Editor {
    fn new() -> Editor {
        Editor {
            state: EditorState::Normal,
            content: String::new(),
            cursor_position: 0,
        }
    }
    fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::MoveCursor(direction, count) => self.move_cursor(direction, count),
            EditorCommand::InsertText(text) => self.insert_text(text),
            EditorCommand::DeleteText(count) => self.delete_text(count),
            EditorCommand::ChangeMode(new_state) => self.state = new_state,
            EditorCommand::Save => self.save(),
            EditorCommand::Quit => println!("Quitting..."),
        }
    }
    fn move_cursor(&mut self, direction: Direction, count: usize) {
        match direction {
            Direction::Up => println!("Moving cursor up by {}", count),
            Direction::Down => println!("Moving cursor down by {}", count),
            Direction::Left => {
                if self.cursor_position >= count {
                    self.cursor_position -= count;
                } else {
                    self.cursor_position = 0;
                }
                println!("Moving cursor left by {}. New position: {}", count, self.cursor_position);
            },
            Direction::Right => {
                self.cursor_position += count;
                println!("Moving cursor right by {}. New position: {}", count, self.cursor_position);
            },
        }
    }
    fn insert_text(&mut self, text: String) {
        match self.state {
            EditorState::Insert => {
                self.content.insert_str(self.cursor_position, &text);
                self.cursor_position += text.len();
                println!("Inserted text. Content: {}", self.content);
            },
            _ => println!("Can only insert text in Insert mode"),
        }
    }
    fn delete_text(&mut self, count: usize) {
        if self.cursor_position >= count && !self.content.is_empty() {
            let end = self.cursor_position;
            let start = end - count;
            self.content.replace_range(start..end, "");
            self.cursor_position = start;
            println!("Deleted {} characters. Content: {}", count, self.content);
        } else {
            println!("Cannot delete {} characters from position {}", count, self.cursor_position);
        }
    }
    fn save(&self) {
        println!("Saving content: {}", self.content);
    }
}
 
fn main() {
    let mut editor = Editor::new();
    
    // Change to insert mode
    editor.handle_command(EditorCommand::ChangeMode(EditorState::Insert));
    
    // Insert some text
    editor.handle_command(EditorCommand::InsertText(String::from("Hello, world!")));
    
    // Move cursor left
    editor.handle_command(EditorCommand::MoveCursor(Direction::Left, 7));
    
    // Delete some text
    editor.handle_command(EditorCommand::DeleteText(5));
    
    // Insert more text
    editor.handle_command(EditorCommand::InsertText(String::from("Rust ")));
    
    // Save the content
    editor.handle_command(EditorCommand::Save);
    
    // Quit
    editor.handle_command(EditorCommand::Quit);
}
 
