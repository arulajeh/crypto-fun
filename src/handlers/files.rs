use crate::constant::response_code::{GENERAL_ERROR_CODE, NOT_FOUND_CODE};
use crate::utils::commons::construct_response;
use actix_web::{get, web, HttpResponse, Responder};
use reqwest::Client;
use serde_json::Value;
use std::env;

#[get("/logo/{file_name}")]
pub async fn proxy_image(path: web::Path<String>) -> impl Responder {
    let file_name = path.into_inner();
    let source_host = env::var("DATA_SOURCE_HOST").expect("DATA_SOURCE_HOST must be set");
    let source_url = format!("{}/data/logos/{}", source_host, file_name);
    let content_type = match file_name.split('.').last() {
        Some("png") => "image/png",
        Some("jpg") => "image/jpeg",
        Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        _ => "image/png",
    };

    let client = Client::new();
    match client.get(&source_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let bytes = response.bytes().await.unwrap_or_default();
                HttpResponse::Ok().content_type(content_type).body(bytes)
            } else {
                HttpResponse::NotFound().json(construct_response::<Value>(
                    None,
                    "File not found",
                    NOT_FOUND_CODE,
                ))
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(construct_response::<Value>(
            None,
            &e.to_string(),
            GENERAL_ERROR_CODE,
        )),
    }
}
