use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GetChartsRequest {
    pub from: String,
    pub to: String,
    pub interval: String,
}