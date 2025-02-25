use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}