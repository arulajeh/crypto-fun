mod models;
mod handlers;

use actix_web::{web, App, HttpServer};
use tokio;

use crate::handlers::price::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/fetch", web::get().to(fetch_crypto_data))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

