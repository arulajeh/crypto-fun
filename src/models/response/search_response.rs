use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub id: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub rank: u32,
    pub price: Option<f64>,
    pub image: Option<String>,
}
