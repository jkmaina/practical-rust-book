// TaskRust API - A RESTful web service for task management
// Built with Rust and Actix Web framework
// Demonstrates modern web service development with type safety and performance
// Provides CRUD operations for tasks with JSON API

mod models;
mod handlers;
mod repository;
mod errors;
mod config;

use actix_web::{web, App, HttpServer, middleware::Logger};
use repository::TaskRepository;
use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = Config::from_env();
    
    // Initialize logger
    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or(&config.log_level)
    );
    
    // Print startup information
    println!("🚀 Starting TaskRust API Server");
    config.print_info();
    
    // Create task repository and add sample data
    let task_repo = TaskRepository::new();
    task_repo.add_sample_data();
    
    println!("📝 Added sample tasks for testing");
    println!("🌐 Server will be available at: http://{}", config.server_addr);
    println!("📚 API documentation available at: http://{}/", config.server_addr);
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(task_repo.clone()))
            .route("/", web::get().to(handlers::api_info))
            .service(
                web::scope("/tasks")
                    .route("", web::get().to(handlers::get_tasks))
                    .route("", web::post().to(handlers::create_task))
                    .route("/{id}", web::get().to(handlers::get_task))
                    .route("/{id}", web::put().to(handlers::update_task))
                    .route("/{id}", web::delete().to(handlers::delete_task))
            )
    })
    .bind(&config.server_addr)?
    .run()
    .await
}