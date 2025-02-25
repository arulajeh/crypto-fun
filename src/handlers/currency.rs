use crate::database::collections::currency_collection::CurrencyCollection;
use crate::models::response::api_response::ApiResponse;
use crate::models::response::currency_response::CurrencyResponse;
use actix_web::{get, web, HttpResponse, Responder};
use bson::doc;
use futures::TryStreamExt;
use mongodb::{Collection, Cursor};

#[get("/currency")]
pub async fn get_currency(db: web::Data<mongodb::Database>) -> impl Responder {
    let collection: Collection<CurrencyCollection> = db.collection("currency");

    // Ambil data dari MongoDB
    let cursor: Cursor<CurrencyCollection> = match collection.find(doc! {}).await {
        Ok(cursor) => cursor,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<Vec<CurrencyResponse>> {
                status: false,
                message: "Failed to fetch data".to_string(),
                data: None,
            })
        }
    };

    // Convert MongoDB Cursor ke Vec
    let currencies: Vec<CurrencyCollection> = match cursor.try_collect().await {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<Vec<CurrencyResponse>> {
                status: false,
                message: "Error processing data".to_string(),
                data: None,
            })
        }
    };

    // Mapping data ke response struct
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
}
