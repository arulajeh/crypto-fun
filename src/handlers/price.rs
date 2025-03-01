use crate::constant::response_code::{GENERAL_ERROR_CODE, SUCCESS_CODE, SUCCESS_MESSAGE};
use crate::models::request::charts_request::GetChartsRequest;
use crate::models::request::price_request::GetPriceRequest;
use crate::models::response::chart_response::ChartResponse;
use crate::models::response::price_response::PriceResponse;
use crate::repositories::AppRepositories;
use crate::utils::commons::{calculate_pagination, construct_pagination_response, construct_response, pagination_response};
use actix_web::{post, web, HttpResponse, Responder};
use bson::doc;
use serde_json::Value;

#[post("/price")]
pub async fn get_price(
    request: web::Json<GetPriceRequest>,
    repository: web::Data<AppRepositories>,
) -> impl Responder {
    let (skip, limit, page) = calculate_pagination(request.pagination.clone());
    match repository
        .price
        .get_prices_by_currency_paginated(&request.currency, skip, limit)
        .await
    {
        Ok(data) => {
            let total_data = repository.price.counts(doc! {"currency": &request.currency}).await.unwrap_or(0);
            let list_data: Vec<PriceResponse> = data
                .into_iter()
                .map(|data| PriceResponse {
                    id: data.id,
                    name: data.name,
                    slug: data.slug,
                    symbol: data.symbol,
                    dominance: data.dominance,
                    image: data.image,
                    rank: data.rank,
                    stable: data.stable,
                    price: data.price,
                    marketcap: data.marketcap,
                    volume: data.volume,
                    cg_id: data.cg_id,
                    symbols: data.symbols,
                    performance: data.performance,
                    rank_diffs: data.rank_diffs,
                    exchange_prices: data.exchange_prices,
                })
                .collect();
            HttpResponse::Ok().json(construct_pagination_response::<Vec<PriceResponse>>(
                Some(list_data),
                SUCCESS_MESSAGE,
                SUCCESS_CODE,
                pagination_response(total_data, page, limit),
            ))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(construct_response::<Value>(
                None,
                &e.to_string(),
                GENERAL_ERROR_CODE,
            ))
        },
    }
}

#[post("/charts")]
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
        Ok(response) => match response.json::<Vec<ChartResponse>>().await {
            Ok(data) => HttpResponse::Ok().json(construct_response::<Vec<ChartResponse>>(
                Some(data),
                SUCCESS_MESSAGE,
                SUCCESS_CODE,
            )),
            Err(e) => {
                HttpResponse::InternalServerError().json(construct_response::<Value>(
                    None,
                    &e.to_string(),
                    GENERAL_ERROR_CODE,
                ))
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(construct_response::<Value>(
                None,
                &e.to_string(),
                GENERAL_ERROR_CODE,
            ))
        },
    }
}
