mod converter;
mod handlers;
mod models;
mod progress;
mod websocket;
mod routes;

use actix_web::{App, HttpServer, middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create uploads directory if it doesn't exist
    std::fs::create_dir_all("./uploads")?;
    std::fs::create_dir_all("./static")?;

    println!("🚀 Server starting at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes::configure_routes)
            .configure(routes::configure_static_files)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}