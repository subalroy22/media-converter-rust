use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionResponse {
    pub session_id: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressMessage {
    pub progress: f32,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}