use crate::models::dto::crypto_bubble_response_dto::Performance;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BubbleResponse {
    pub id: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub rank: u32,
    pub price: Option<f64>,
    pub marketcap: Option<u64>,
    pub volume: Option<u64>,
    pub dominance: Option<f64>,
    pub performance: Performance,
    pub image: Option<String>,
    pub stable: bool,
}
