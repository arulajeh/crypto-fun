use crate::constant::response_code::{GENERAL_ERROR_CODE, SUCCESS_CODE, SUCCESS_MESSAGE};
use crate::models::response::currency_response::CurrencyResponse;
use crate::repositories::AppRepositories;
use crate::utils::commons::construct_response;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::Value;

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
            HttpResponse::Ok().json(construct_response::<Vec<CurrencyResponse>>(
                Some(list_currency),
                SUCCESS_MESSAGE,
                SUCCESS_CODE,
            ))
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(construct_response::<Value>(None, &e.to_string(), GENERAL_ERROR_CODE)),
    }
}
