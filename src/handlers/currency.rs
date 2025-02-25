use actix_web::{HttpResponse, Responder};
use crate::models::response::api_response::ApiResponse;

pub async fn get_currency() -> impl Responder {
    let currencies = vec![
        "BRL".to_string(),
        "USD".to_string(),
        "JPY".to_string(),
        "EUR".to_string(),
        "CAD".to_string(),
        "AUD".to_string(),
        "PLN".to_string(),
        "GBP".to_string(),
        "CHF".to_string(),
        "KRW".to_string(),
        "ZAR".to_string(),
        "BTC".to_string(),
        "ETH".to_string(),
        "TRY".to_string(),
        "SOL".to_string(),
        "RUB".to_string(),
        "INR".to_string(),
    ];
    HttpResponse::Ok().json(ApiResponse::<Vec<String>> {
        status: true,
        message: "Success".to_string(),
        data: Option::from(currencies),
    })
}