use crate::models::request::pagination_request::PaginationRequest;

pub fn calculate_pagination(pagination: Option<PaginationRequest>) -> (u64, u64) {
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
    (skip, limit)
}