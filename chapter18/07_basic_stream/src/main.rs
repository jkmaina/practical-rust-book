// Async streams example demonstrating asynchronous iteration over data
// Uses tokio-stream's StreamExt for .next().await

use tokio_stream::{iter, StreamExt};

#[tokio::main]
async fn main() {
    let mut values = iter([1, 2, 3, 4, 5]);

    while let Some(value) = values.next().await {
        println!("Got value: {}", value);
    }
}
