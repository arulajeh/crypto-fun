use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use crate::models::dto::crypto_bubble_response_dto::{Performance, RankDiffs};

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceCollection {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub symbol: Option<String>,
    pub dominance: Option<f64>,
    pub image: Option<String>,
    pub rank: u32,
    pub stable: bool,
    pub price: Option<f64>,
    pub marketcap: Option<u64>,
    pub volume: Option<u64>,
    pub cg_id: Option<String>,
    pub symbols: HashMap<String, String>,
    pub performance: Performance,
    #[serde(rename = "rankDiffs")]
    pub rank_diffs: RankDiffs,
    #[serde(rename = "exchangePrices")]
    pub exchange_prices: HashMap<String, Option<f64>>,
}