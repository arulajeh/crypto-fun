mod database;
mod handlers;
mod models;
mod repositories;
mod scheduler;
mod utils;

use crate::database::connect_mongodb;
use crate::handlers::currency::get_currency;
use crate::handlers::default::default;
use crate::handlers::price::{get_charts, get_price};
use crate::repositories::AppRepositories;
use crate::scheduler::bubble_price::fetch_bubble_price;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let db = Arc::new(
        connect_mongodb()
            .await
            .expect("Failed to connect to MongoDB"),
    );

    let db_cloned = Arc::clone(&db);
    tokio::spawn(async move {
        fetch_bubble_price(db_cloned).await;
    });
    let app_repositories = Arc::new(AppRepositories::new(Arc::clone(&db)));
    println!("Server running on {}", &port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_repositories.clone()))
            .service(get_currency)
            .service(get_price)
            .service(get_charts)
            .default_service(web::route().to(default))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
