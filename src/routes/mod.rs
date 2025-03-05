mod currency;
mod files;
mod price;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/currency").configure(currency::currency_routes))
        .service(web::scope("/price").configure(price::price_routes))
        .service(web::scope("/files").configure(files::files_routes));
}
