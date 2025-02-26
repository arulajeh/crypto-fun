use serde::{Deserialize, Serialize};
use crate::models::request::pagination_request::PaginationRequest;

#[derive(Deserialize, Serialize, Clone)]
pub struct GetPriceRequest {
    pub currency: String,
    pub pagination: Option<PaginationRequest>,
}
