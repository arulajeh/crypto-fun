use crate::handlers::price::{get_bubbles, get_charts, get_price};
use actix_web::web;

pub fn price_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_price)
        .service(get_charts)
        .service(get_bubbles);
}
