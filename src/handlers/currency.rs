use crate::models::response::api_response::ApiResponse;
use crate::models::response::currency_response::CurrencyResponse;
use crate::repositories::AppRepositories;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/currency")]
pub async fn get_currency(repository: web::Data<AppRepositories>) -> impl Responder {
    match repository.currency.get_all_currency().await {
        Ok(currencies) => {
            let list_currency: Vec<CurrencyResponse> = currencies
                .into_iter()
                .map(|data| CurrencyResponse {
                    id: data.id.unwrap().to_hex(),
                    label: data.label,
                    name: data.name,
                    symbol: data.symbol,
                    type_: data.c_type,
                })
                .collect();
            HttpResponse::Ok().json(ApiResponse::<Vec<CurrencyResponse>> {
                status: true,
                message: "Success".to_string(),
                data: Some(list_currency),
            })
        },
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::<Vec<CurrencyResponse>> {
                status: false,
                message: "Failed to fetch data".to_string(),
                data: None,
            })
        }
    }
}
