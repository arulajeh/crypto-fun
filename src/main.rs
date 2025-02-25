mod database;
mod handlers;
mod models;
mod scheduler;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;
use tokio;

use crate::database::database::connect_mongodb;
use crate::handlers::currency::get_currency;
use crate::handlers::default::default;
use crate::handlers::price::{get_charts, get_price};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let db = connect_mongodb()
        .await
        .expect("Failed to connect to MongoDB");
    println!("Server running on {}", &port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(get_currency)
            .route("/price", web::post().to(get_price))
            .route("/charts", web::post().to(get_charts))
            .default_service(web::route().to(default))
    })
    .bind("0.0.0.0:".to_string() + &port)?
    .run()
    .await
}
