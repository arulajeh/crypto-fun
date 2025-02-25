use actix_web::{HttpResponse, Responder};
use serde_json::Value;
use crate::models::response::api_response::ApiResponse;

pub async fn default() -> impl Responder {
    HttpResponse::NotFound().json(ApiResponse::<Value> {
        status: false,
        message: "Not Found".to_string(),
        data: None,
    })
}