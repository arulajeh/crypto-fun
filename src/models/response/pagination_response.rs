use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationResponse {
    #[serde(rename = "totalPage")]
    pub total_page: Option<u64>,
    #[serde(rename = "totalData")]
    pub total_data: Option<u64>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
}