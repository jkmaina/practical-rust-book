// Filter even numbers and multiply by 2
use tokio_stream::{iter, StreamExt};

#[tokio::main]
async fn main() {
    let mut stream = iter([1, 2, 3, 4, 5, 6])
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2);

    while let Some(value) = stream.next().await {
        println!("Got: {}", value);
    }
}