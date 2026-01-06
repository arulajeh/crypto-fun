use crate::constant::response_code::{GENERAL_ERROR_CODE, SUCCESS_CODE, SUCCESS_MESSAGE};
use crate::models::request::bubble_request::BubbleRequest;
use crate::models::request::charts_request::GetChartsRequest;
use crate::models::request::price_request::GetPriceRequest;
use crate::models::response::bubble_response::BubbleResponse;
use crate::models::response::chart_response::{ChartResponse, ChartTicks};
use crate::models::response::price_response::PriceResponse;
use crate::models::response::search_response::SearchResponse;
use crate::repositories::AppRepositories;
use crate::utils::commons::{
    calculate_pagination, construct_pagination_response, construct_response, pagination_response,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use bson::doc;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[post("")]
pub async fn get_price(
    request: web::Json<GetPriceRequest>,
    repository: web::Data<AppRepositories>,
) -> impl Responder {
    println!("incoming request get price {:?}", request);
    let (skip, limit, page) = calculate_pagination(request.pagination.clone());
    match repository
        .price
        .get_prices_by_currency_paginated(&request.currency, skip, limit)
        .await
    {
        Ok(data) => {
            let total_data = repository
                .price
                .counts(doc! {"currency": &request.currency})
                .await
                .unwrap_or(0);
            let base_path = env::var("BASE_PATH").expect("BASE_PATH must be set");
            let list_data: Vec<PriceResponse> = data
                .into_iter()
                .map(|data| PriceResponse {
                    id: data.id,
                    name: data.name,
                    slug: data.slug,
                    symbol: data.symbol,
                    dominance: data.dominance,
                    image: match data.image {
                        Some(image) => format!("{}/{}", base_path, image).into(),
                        None => Option::from("".to_string()),
                    },
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
        Err(e) => HttpResponse::InternalServerError().json(construct_response::<Value>(
            None,
            &e.to_string(),
            GENERAL_ERROR_CODE,
        )),
    }
}

#[post("/charts")]
pub async fn get_charts(request: web::Json<GetChartsRequest>) -> impl Responder {
    println!("incoming request get charts {:?}", request);
    let source_host = env::var("DATA_SOURCE_HOST").expect("DATA_SOURCE_HOST must be set");
    let intervals = vec!["hour", "day", "week", "month", "year"];
    let futures = intervals.into_iter().map(|interval| {
        let source_host = source_host.clone();
        let from = request.from.to_lowercase();
        let to = request.to.to_uppercase();

        tokio::spawn(async move {
            let url = format!(
                "{}/data/charts/{}/{}/{}.json",
                source_host, interval, from, to
            );

            let data = match reqwest::get(&url).await {
                Ok(response) => (response.json::<Vec<ChartTicks>>().await).unwrap_or_default(),
                Err(_) => vec![],
            };

            ChartResponse {
                interval: interval.to_string(),
                data,
            }
        })
    });

    let results: Vec<ChartResponse> = join_all(futures)
        .await
        .into_iter()
        .filter_map(|res| res.ok())
        .collect();
    HttpResponse::Ok().json(construct_response::<Vec<ChartResponse>>(
        Some(results),
        SUCCESS_MESSAGE,
        SUCCESS_CODE,
    ))
}

#[post("/bubbles")]
pub async fn get_bubbles(
    request: web::Json<BubbleRequest>,
    repository: web::Data<AppRepositories>,
) -> impl Responder {
    println!("incoming request get bubbles {:?}", request);

    match repository
        .price
        .get_bubbles_data(&request.currency.to_lowercase(), request.limit, request.exclude_stablecoins)
        .await
    {
        Ok(data) => {
            let base_path = env::var("BASE_PATH").expect("BASE_PATH must be set");
            let bubbles: Vec<BubbleResponse> = data
                .into_iter()
                .map(|item| BubbleResponse {
                    id: item.id,
                    name: item.name,
                    symbol: item.symbol,
                    rank: item.rank,
                    price: item.price,
                    marketcap: item.marketcap,
                    volume: item.volume,
                    dominance: item.dominance,
                    performance: item.performance,
                    image: match item.image {
                        Some(image) => format!("{}/{}", base_path, image).into(),
                        None => Option::from("".to_string()),
                    },
                    stable: item.stable,
                })
                .collect();

            HttpResponse::Ok().json(construct_response::<Vec<BubbleResponse>>(
                Some(bubbles),
                SUCCESS_MESSAGE,
                SUCCESS_CODE,
            ))
        }
        Err(e) => HttpResponse::InternalServerError().json(construct_response::<Value>(
            None,
            &e.to_string(),
            GENERAL_ERROR_CODE,
        )),
    }
}

#[derive(Deserialize, Serialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_currency")]
    pub currency: String,
    #[serde(default = "default_search_limit")]
    pub limit: u64,
}

fn default_currency() -> String {
    "USD".to_string()
}

fn default_search_limit() -> u64 {
    10
}

#[get("/search")]
pub async fn search_price(
    query: web::Query<SearchQuery>,
    repository: web::Data<AppRepositories>,
) -> impl Responder {
    println!("incoming search request: query={}, currency={}", query.q, query.currency);

    if query.q.trim().is_empty() {
        return HttpResponse::BadRequest().json(construct_response::<Value>(
            None,
            "Search query cannot be empty",
            GENERAL_ERROR_CODE,
        ));
    }

    match repository
        .price
        .search_crypto(&query.q, &query.currency.to_lowercase(), query.limit)
        .await
    {
        Ok(data) => {
            let base_path = env::var("BASE_PATH").expect("BASE_PATH must be set");
            let results: Vec<SearchResponse> = data
                .into_iter()
                .map(|item| SearchResponse {
                    id: item.id,
                    name: item.name,
                    symbol: item.symbol,
                    rank: item.rank,
                    price: item.price,
                    image: match item.image {
                        Some(image) => format!("{}/{}", base_path, image).into(),
                        None => Option::from("".to_string()),
                    },
                })
                .collect();

            HttpResponse::Ok().json(construct_response::<Vec<SearchResponse>>(
                Some(results),
                SUCCESS_MESSAGE,
                SUCCESS_CODE,
            ))
        }
        Err(e) => HttpResponse::InternalServerError().json(construct_response::<Value>(
            None,
            &e.to_string(),
            GENERAL_ERROR_CODE,
        )),
    }
}
