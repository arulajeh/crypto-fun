mod models;
mod handlers;

use actix_web::{web, App, HttpServer};
use tokio;

use crate::handlers::price::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Listen at port 8080");
    HttpServer::new(|| {
        App::new()
            .route("/fetch", web::get().to(fetch_crypto_data))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

