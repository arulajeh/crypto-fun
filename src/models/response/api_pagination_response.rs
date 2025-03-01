use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::models::response::pagination_response::PaginationResponse;

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T = Value> {
    pub status: bool,
    pub message: String,
    pub code: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    pub payload: Option<T>,
    pub pagination: Option<PaginationResponse>,
}