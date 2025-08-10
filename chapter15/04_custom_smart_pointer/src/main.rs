// Drop trait: enables custom cleanup code when values go out of scope
// Implements RAII (Resource Acquisition Is Initialization) pattern
// Automatically called by Rust - cannot be called manually (use std::mem::drop instead)

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// Example with file-like resource
struct FileResource {
    filename: String,
}

impl FileResource {
    fn new(filename: &str) -> Self {
        println!("Opening file: {}", filename);
        FileResource {
            filename: filename.to_string(),
        }
    }
}

impl Drop for FileResource {
    fn drop(&mut self) {
        println!("Closing file: {}", self.filename);
    }
}

fn main() {
    println!("=== Automatic Drop on Scope Exit ===");
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    } // c and d are dropped here in reverse order (d, then c)
    
    println!("\n=== Manual Drop with std::mem::drop ===");
    let e = CustomSmartPointer {
        data: String::from("early drop"),
    };
    println!("Before manual drop");
    drop(e);  // Manually drop e early
    println!("After manual drop");
    
    println!("\n=== File Resource Example ===");
    {
        let _file1 = FileResource::new("config.txt");
        let _file2 = FileResource::new("data.txt");
        println!("Files are open");
    } // Files automatically closed here
    
    println!("\n=== Drop Order Demonstration ===");
    let first = CustomSmartPointer {
        data: String::from("first"),
    };
    let second = CustomSmartPointer {
        data: String::from("second"),
    };
    let third = CustomSmartPointer {
        data: String::from("third"),
    };
    println!("All pointers created - they'll drop in reverse order");
    // Drops in reverse order: third, second, first
}
