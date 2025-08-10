// Async trait example using async_trait crate
// Demonstrates how to define and implement async methods in traits
// async_trait macro enables async functions in traits (not natively supported)
// Useful for defining async interfaces and polymorphic async behavior

use async_trait::async_trait;

#[async_trait]
trait AsyncTrait {
    async fn foo(&self);
}

struct MyStruct;

#[async_trait]
impl AsyncTrait for MyStruct {
    async fn foo(&self) {
        println!("Hello from async trait method!");
    }
}

#[tokio::main]
async fn main() {
    let my_struct = MyStruct;
    
    // Call the async trait method
    my_struct.foo().await;
    
    // Demonstrate polymorphism with trait objects
    let trait_obj: &dyn AsyncTrait = &my_struct;
    trait_obj.foo().await;
}