use crate::handlers::currency::get_currency;
use actix_web::web;

pub fn currency_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_currency);
}
