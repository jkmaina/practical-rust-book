// This program demonstrates the use of labeled loops in Rust.
// It shows how to break out of nested loops using labels.
// The outer loop is labeled 'outer and the inner loop is labeled 'inner.
// When the inner loop breaks, it will exit the outer loop as well.
// The program will print messages indicating when it enters and exits each loop.
// The final message will confirm that the outer loop has been exited.

fn main() {
    'outer: loop {
        println!("Entered the outer loop");
        
        'inner: loop {
            println!("Entered the inner loop");
            
            // This breaks the outer loop
            break 'outer;
            
            // This line is never reached
            println!("This point will never be reached");
        }
        
        // This line is never reached because we broke the outer loop
        println!("This point will never be reached");
    }
    
    println!("Exited the outer loop");
}
