use crate::models::response::pagination_response::PaginationResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T = Value> {
    pub status: bool,
    pub message: String,
    pub payload: Option<T>,
    pub code: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiPaginationResponse<T = Value> {
    pub status: bool,
    pub message: String,
    pub code: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    pub payload: Option<T>,
    pub pagination: PaginationResponse,
}
