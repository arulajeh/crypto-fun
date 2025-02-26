use crate::database::collections::currency_collection::CurrencyCollection;
use bson::doc;
use futures::TryStreamExt;
use mongodb::{Collection, Database};

pub struct CurrencyRepository {
    collection: Collection<CurrencyCollection>,
}

impl CurrencyRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection::<CurrencyCollection>("currency");
        Self { collection }
    }

    pub async fn get_all_currency(&self) -> Result<Vec<CurrencyCollection>, mongodb::error::Error> {
        let cursor = self.collection.find(doc! {}).await?;
        let currencies: Vec<CurrencyCollection> = cursor.try_collect().await?;
        Ok(currencies)
    }
}