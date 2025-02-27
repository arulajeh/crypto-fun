use crate::constant::response_code::{NOT_FOUND_CODE, NOT_FOUND_MESSAGE};
use crate::utils::commons::construct_response;
use actix_web::{HttpResponse, Responder};
use serde_json::Value;

pub async fn default() -> impl Responder {
    HttpResponse::NotFound().json(construct_response::<Value>(
        None,
        NOT_FOUND_MESSAGE,
        NOT_FOUND_CODE,
    ))
}
