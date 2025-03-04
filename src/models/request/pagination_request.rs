use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PaginationRequest {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}