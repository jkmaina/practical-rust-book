// Async web server example using Warp framework
// Demonstrates shared state management with Arc<RwLock<T>>
// Shows REST API endpoints with async handlers
// Includes sample data for testing the server functionality

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

type Items = HashMap<String, i32>;
type ItemsDb = Arc<RwLock<Items>>;

#[tokio::main]
async fn main() {
    // Create shared state with sample data
    let mut initial_items = HashMap::new();
    initial_items.insert("apples".to_string(), 10);
    initial_items.insert("bananas".to_string(), 5);
    initial_items.insert("oranges".to_string(), 8);
    
    let db: ItemsDb = Arc::new(RwLock::new(initial_items));
    
    println!("Sample data loaded:");
    println!("- apples: 10");
    println!("- bananas: 5");
    println!("- oranges: 8");
    
    // Define routes
    let root = warp::path::end()
        .and(warp::get())
        .map(|| warp::reply::html(
            r#"<h1>Async Web Server</h1>
            <h2>Available Endpoints:</h2>
            <ul>
                <li><strong>GET /</strong> - This help page</li>
                <li><strong>GET /hello/{name}</strong> - Returns greeting</li>
                <li><strong>GET /items</strong> - Returns all items</li>
                <li><strong>POST /items/{name}</strong> - Adds item with JSON body {"count": number}</li>
            </ul>
            <h2>Sample Data:</h2>
            <ul>
                <li>apples: 10</li>
                <li>bananas: 5</li>
                <li>oranges: 8</li>
            </ul>"#
        ));
    
    let hello = warp::path!("hello" / String)
        .and(warp::get())
        .map(|name| format!("Hello, {}!", name));
    
    println!("\nAvailable endpoints:");
    println!("- GET / - API documentation");
    println!("- GET /hello/{{name}} - Returns greeting");
    println!("- GET /items - Returns all items");
    println!("- POST /items/{{name}} - Adds item with JSON body {{\"count\": number}}");
    
    let get_items = warp::path("items")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(get_all_items);
    
    let add_item = warp::path!("items" / String)
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(add_item);
    
    let routes = root.or(hello).or(get_items).or(add_item);
    
    // Start the server
    println!("Server starting at http://localhost:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
fn with_db(db: ItemsDb) -> impl Filter<Extract = (ItemsDb,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
async fn get_all_items(db: ItemsDb) -> Result<impl warp::Reply, warp::Rejection> {
    let items = db.read().await;
    Ok(warp::reply::json(&*items))
}
async fn add_item(name: String, count: i32, db: ItemsDb) -> Result<impl warp::Reply, warp::Rejection> {
    let mut items = db.write().await;
    items.insert(name, count);
    Ok(warp::reply::with_status(
        "Added item",
        warp::http::StatusCode::CREATED,
    ))
}
 
