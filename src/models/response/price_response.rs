use crate::models::dto::crypto_bubble_response_dto::{
    ExchangePrices, Performance, RankDiffs, Symbols,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceResponse {
    pub stable: bool,
    pub id: Option<String>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub symbol: Option<String>,
    pub dominance: Option<f64>,
    pub image: Option<String>,
    pub rank: u32,
    pub price: Option<f64>,
    pub marketcap: Option<u64>,
    pub volume: Option<u64>,
    pub cg_id: Option<String>,
    pub symbols: Symbols,
    pub performance: Performance,
    #[serde(rename = "rankDiffs")]
    pub rank_diffs: RankDiffs,
    #[serde(rename = "exchangePrices")]
    pub exchange_prices: ExchangePrices,
}
