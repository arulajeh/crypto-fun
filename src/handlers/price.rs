use crate::models::dto::chart_response_dto::ChartResponseDto;
use crate::models::dto::crypto_bubble_response_dto::CryptoDataDto;
use crate::models::request::charts_request::GetChartsRequest;
use crate::models::request::price_request::GetPriceRequest;
use crate::models::response::api_response::ApiResponse;
use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;

pub async fn get_price(request: web::Json<GetPriceRequest>) -> impl Responder {
    let url = format!(
        "https://cryptobubbles.net/backend/data/bubbles1000.{}.json",
        request.currency.to_lowercase()
    );
    match reqwest::get(url).await {
        Ok(response) => match response.json::<Vec<CryptoDataDto>>().await {
            Ok(data) => HttpResponse::Ok().json(ApiResponse::<Vec<CryptoDataDto>> {
                status: true,
                message: "Success".to_string(),
                data: Option::from(data),
            }),
            Err(e) => {
                println!("Error: {:?}", e);
                HttpResponse::InternalServerError().json(ApiResponse::<Value> {
                    status: false,
                    message: "Failed to fetch data".to_string(),
                    data: None,
                })
            }
        },
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<Value> {
            status: false,
            message: "Failed to fetch data".to_string(),
            data: None,
        }),
    }
}

pub async fn get_charts(request: web::Json<GetChartsRequest>) -> impl Responder {
    let url = format!(
        "https://cryptobubbles.net/backend/data/charts/{}/{}/{}.json",
        request.interval,
        request.from.to_lowercase(),
        request.to.to_uppercase()
    );
    // create urls yang bedanya pada satuan datanya hour, day, week, month, year
    // let urls = vec![
    //     format!(
    //         "https://cryptobubbles.net/backend/data/charts/hour/{}/{}.json",
    //         request.from.to_lowercase(),
    //         request.to.to_uppercase()
    //     ),
    //     format!(
    //         "https://cryptobubbles.net/backend/data/charts/day/{}/{}.json",
    //         request.from.to_lowercase(),
    //         request.to.to_uppercase()
    //     ),
    //     format!(
    //         "https://cryptobubbles.net/backend/data/charts/week/{}/{}.json",
    //         request.from.to_lowercase(),
    //         request.to.to_uppercase()
    //     ),
    //     format!(
    //         "https://cryptobubbles.net/backend/data/charts/month/{}/{}.json",
    //         request.from.to_lowercase(),
    //         request.to.to_uppercase()
    //     ),
    //     format!(
    //         "https://cryptobubbles.net/backend/data/charts/year/{}/{}.json",
    //         request.from.to_lowercase(),
    //         request.to.to_uppercase()
    //     ),
    // ];

    match reqwest::get(url).await {
        Ok(response) => match response.json::<Vec<ChartResponseDto>>().await {
            Ok(data) => HttpResponse::Ok().json(ApiResponse::<Vec<ChartResponseDto>> {
                status: true,
                message: "Success".to_string(),
                data: Option::from(data),
            }),
            Err(e) => {
                println!("Error: {:?}", e);
                HttpResponse::InternalServerError().json(ApiResponse::<Value> {
                    status: false,
                    message: "Failed to fetch data".to_string(),
                    data: None,
                })
            }
        },
        Err(_) => HttpResponse::InternalServerError().json(ApiResponse::<Value> {
            status: false,
            message: "Failed to fetch data".to_string(),
            data: None,
        }),
    }
}
