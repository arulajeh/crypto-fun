use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PaginationRequest {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}