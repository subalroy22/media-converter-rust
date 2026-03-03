use actix_web::{web, HttpResponse};
use actix_files as fs;
use crate::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/upload", web::post().to(handlers::upload_file))
            .route("/ws/{session_id}", web::get().to(handlers::ws_convert))
            .route("/download/{session_id}", web::get().to(handlers::download_file))
    );
}

pub fn configure_static_files(cfg: &mut web::ServiceConfig) {
    cfg.service(fs::Files::new("/", "./static").index_file("index.html"));
}
