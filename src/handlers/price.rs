use actix_web::{HttpResponse, Responder};
use crate::models::response::api_response::ApiResponse;
use crate::models::response::crypto_bubble_response::CryptoData;

pub async fn fetch_crypto_data() -> impl Responder {
    let url = "https://cryptobubbles.net/backend/data/bubbles1000.usd.json";

    match reqwest::get(url).await {
        Ok(response) => match response.json::<Vec<CryptoData>>().await {
            Ok(data) => HttpResponse::Ok().json(ApiResponse {
                status: true,
                message: "Success".to_string(),
                data: Option::from(serde_json::to_value(data).unwrap()),
            }),
            Err(e) => {
                println!("Error: {:?}", e);
                HttpResponse::InternalServerError().json(ApiResponse {
                    status: false,
                    message: "Failed to fetch data".to_string(),
                    data: None,
                })
            },
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse {
                status: false,
                message: "Failed to fetch data".to_string(),
                data: None,
            })
        },
    }
}