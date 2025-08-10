use actix_web::{get, App, HttpResponse, HttpServer, Responder};
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, containerized Rust!")
}
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(health)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
 
