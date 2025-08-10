// A simple generic data buffer implementation in Rust

struct Buffer<T> {
    data: Vec<T>,
    capacity: usize,
}
impl<T> Buffer<T> {
    fn new(capacity: usize) -> Buffer<T> {
        Buffer {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }
    
    fn add(&mut self, value: T) -> Result<(), String> {
        if self.data.len() >= self.capacity {
            return Err("Buffer is full".to_string());
        }
        self.data.push(value);
        Ok(())
    }
    
    fn get_all(&self) -> &[T] {
        &self.data
    }
}
fn main() {
    // Integer buffer
    let mut int_buffer = Buffer::new(5);
    int_buffer.add(1).unwrap();
    int_buffer.add(2).unwrap();
    println!("Int buffer: {:?}", int_buffer.get_all());
    
    // String buffer
    let mut string_buffer = Buffer::<String>::new(3);
    string_buffer.add("hello".to_string()).unwrap();
    string_buffer.add("world".to_string()).unwrap();
    println!("String buffer: {:?}", string_buffer.get_all());
}
