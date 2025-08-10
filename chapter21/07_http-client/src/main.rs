use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: u32,
    title: String,
    body: String,
    userId: u32,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Make a GET request to the JSONPlaceholder API
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await?
        .json::<Post>()
        .await?;
    println!("Received post:");
    println!("ID: {}", response.id);
    println!("Title: {}", response.title);
    println!("Body: {}", response.body);
    println!("User ID: {}", response.userId);
    Ok(())
}
