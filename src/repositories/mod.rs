use std::sync::Arc;

pub mod price_repository;
pub mod currency_repository;

#[derive(Clone)]
pub struct AppRepositories {
    pub price: Arc<price_repository::PriceRepository>,
    pub currency: Arc<currency_repository::CurrencyRepository>,
}

impl AppRepositories {
    pub fn new(db: Arc<mongodb::Database>) -> Self {
        Self {
            price: Arc::new(price_repository::PriceRepository::new(&db)),
            currency: Arc::new(currency_repository::CurrencyRepository::new(&db)),
        }
    }
}