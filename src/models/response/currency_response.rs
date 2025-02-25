use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyResponse {
    pub id: String,
    pub label: String,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "type")]
    pub type_: String,
}