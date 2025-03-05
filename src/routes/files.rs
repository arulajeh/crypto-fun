use crate::handlers::files::proxy_logo;
use actix_web::web;

pub fn files_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(proxy_logo);
}