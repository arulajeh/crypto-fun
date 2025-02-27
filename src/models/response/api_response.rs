use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T = Value> {
    pub status: bool,
    pub message: String,
    pub payload: Option<T>,
    pub code: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String
}