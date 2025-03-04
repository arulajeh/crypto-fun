use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetChartsRequest {
    pub from: String,
    pub to: String
}