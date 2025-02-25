mod handlers;
mod models;
mod scheduler;

use actix_web::{web, App, HttpServer};
use tokio;

use crate::handlers::currency::get_currency;
use crate::handlers::default::default;
use crate::handlers::price::{get_charts, get_price};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Listen at port 8080");
    HttpServer::new(|| {
        App::new()
            .route("/currency", web::get().to(get_currency))
            .route("/price", web::post().to(get_price))
            .route("/charts", web::post().to(get_charts))
            .default_service(web::route().to(default))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
