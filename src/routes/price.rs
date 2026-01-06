use crate::handlers::price::{get_bubbles, get_charts, get_price, get_price_detail, search_price};
use actix_web::web;

pub fn price_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_price)
        .service(get_charts)
        .service(get_bubbles)
        .service(search_price)
        .service(get_price_detail); // Must be last to avoid conflict with other routes
}
