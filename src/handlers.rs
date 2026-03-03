use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::io::Write;
use uuid::Uuid;
use crate::models::{ConversionResponse, ErrorResponse};
use crate::websocket::websocket_handler;

pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(item) = payload.next().await {
        let mut field: actix_multipart::Field = item?;
        
        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .get_filename()
            .ok_or_else(|| actix_web::error::ErrorBadRequest("Filename not found"))?;

        if !filename.ends_with(".mp4") {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                error: "Only MP4 files are supported".to_string(),
            }));
        }

        let session_id = Uuid::new_v4().to_string();
        let filepath = format!("./uploads/{}.mp4", session_id);

        let mut f = std::fs::File::create(&filepath)?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data)?;
        }

        return Ok(HttpResponse::Ok().json(ConversionResponse {
            session_id: session_id.clone(),
            message: "File uploaded successfully".to_string(),
        }));
    }

    Ok(HttpResponse::BadRequest().json(ErrorResponse {
        error: "No file uploaded".to_string(),
    }))
}

pub async fn ws_convert(
    req: HttpRequest,
    stream: web::Payload,
    session_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let file_path = format!("./uploads/{}.mp4", session_id.as_str());
    
    if !std::path::Path::new(&file_path).exists() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            error: "File not found".to_string(),
        }));
    }

    let (response, session, msg_stream) = actix_ws::handle(&req, stream)?;

    actix_web::rt::spawn(websocket_handler(
        session,
        msg_stream,
        std::path::PathBuf::from(file_path),
    ));

    Ok(response)
}

pub async fn download_file(session_id: web::Path<String>) -> Result<actix_files::NamedFile, Error> {
    let file_path = format!("./uploads/{}.mp3", session_id.as_str());
    let file = actix_files::NamedFile::open(file_path)?;
    Ok(file.set_content_disposition(actix_web::http::header::ContentDisposition {
        disposition: actix_web::http::header::DispositionType::Attachment,
        parameters: vec![actix_web::http::header::DispositionParam::Filename(format!("converted_{}.mp3", session_id.as_str()))],
    }))
}