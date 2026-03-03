use actix_ws::Message;
use futures_util::StreamExt;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::converter::convert_to_mp3_with_progress;
use crate::models::ProgressMessage;

pub async fn websocket_handler(
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
    file_path: PathBuf,
) {
    let session_arc = Arc::new(Mutex::new(session));
    let session_clone = session_arc.clone();

    // Spawn conversion task in background
    let conversion_task = tokio::spawn(async move {
        let _ = convert_to_mp3_with_progress(&file_path, move |progress, message| {
            let session = session_clone.clone();
            let progress_msg = ProgressMessage {
                progress,
                status: if progress >= 100.0 { "completed".to_string() } else { "processing".to_string() },
                message,
            };

            // Send progress update
            tokio::spawn(async move {
                if let Ok(json) = serde_json::to_string(&progress_msg) {
                    if let Ok(mut sess) = session.try_lock() {
                        let _ = sess.text(json).await;
                    }
                }
            });
        }).await;
    });

    // Handle incoming WebSocket messages
    while let Some(Ok(msg)) = msg_stream.next().await {
        match msg {
            Message::Ping(bytes) => {
                if let Ok(mut sess) = session_arc.try_lock() {
                    let _ = sess.pong(&bytes).await;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // Wait for conversion to complete
    let _ = conversion_task.await;
}