struct IntBuffer {
    data: Vec<i32>,
    capacity: usize,
}
impl IntBuffer {
    fn new(capacity: usize) -> IntBuffer {
        IntBuffer {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }    
    fn add(&mut self, value: i32) -> Result<(), String> {
        if self.data.len() >= self.capacity {
            return Err("Buffer is full".to_string());
        }
        self.data.push(value);
        Ok(())
    }
    
    fn get_all(&self) -> &[i32] {
        &self.data
    }
}