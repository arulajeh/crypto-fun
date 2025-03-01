use crate::models::request::pagination_request::PaginationRequest;
use crate::models::response::api_response::{ApiResponse, ApiPaginationResponse};

use crate::models::response::pagination_response::PaginationResponse;

pub fn calculate_pagination(pagination: Option<PaginationRequest>) -> (u64, u64, u64) {
    let default_limit = 10;
    let default_page = 1;
    let (page, limit) = match pagination {
        Some(p) => (
            p.page.unwrap_or(default_page).max(1),
            p.limit.unwrap_or(default_limit).max(1),
        ),
        None => (default_page, default_limit),
    };
    let skip = (page - 1) * limit;
    (skip, limit, page)
}

pub fn pagination_response(total_data: u64, page: u64, limit: u64) -> PaginationResponse {
    let total_page = if total_data % limit == 0 {
        total_data / limit
    } else {
        total_data / limit + 1
    };
    PaginationResponse {
        total_page: Some(total_page),
        total_data: Some(total_data),
        page: Some(page),
        limit: Some(limit),
    }
}



pub fn construct_response<T>(payload: Option<T>, message: &str, code: &str) -> ApiResponse<T> {
    ApiResponse {
        status: code.starts_with("2"),
        message: message.to_string(),
        payload,
        code: code.to_string(),
        time_stamp: chrono::Utc::now().to_rfc3339(),
    }
}

pub fn construct_pagination_response<T>(payload: Option<T>, message: &str, code: &str, pagination: PaginationResponse) -> ApiPaginationResponse<T> {
    ApiPaginationResponse {
        status: code.starts_with("2"),
        message: message.to_string(),
        payload,
        pagination,
        code: code.to_string(),
        time_stamp: chrono::Utc::now().to_rfc3339(),
    }
}
