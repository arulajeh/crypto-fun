use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BubbleRequest {
    pub currency: String,
    #[serde(default = "default_limit")]
    pub limit: u64,
    #[serde(default)]
    pub exclude_stablecoins: bool,
    pub timeframe: Option<String>,
}

fn default_limit() -> u64 {
    1000
}
