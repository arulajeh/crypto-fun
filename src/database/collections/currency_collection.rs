use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyCollection {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub label: String,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "type")]
    pub c_type: String
}