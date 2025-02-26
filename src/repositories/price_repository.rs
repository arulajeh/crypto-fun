use crate::database::collections::price_collection::PriceCollection;
use bson::{doc, to_document};
use futures::TryStreamExt;
use mongodb::{Collection, Database};

pub struct PriceRepository {
    collection: Collection<PriceCollection>,
}

impl PriceRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection::<PriceCollection>("price");
        Self { collection }
    }

    pub async fn save_or_update(
        &self,
        price: PriceCollection,
    ) -> Result<(), mongodb::error::Error> {
        let price_doc = to_document(&price)?;
        self.collection
            .update_one(
                doc! {"id": price.id, "currency": price.currency},
                doc! {"$set": price_doc},
            )
            .upsert(true)
            .await?;
        Ok(())
    }

    pub async fn get_prices_by_currency_paginated(
        &self,
        currency: &str,
        skip: u64,
        limit: u64,
    ) -> Result<Vec<PriceCollection>, mongodb::error::Error> {
        let cursor = self
            .collection
            .find(doc! {"currency": currency})
            .skip(skip)
            .limit(limit as i64)
            .sort(doc! {"rank": 1})
            .await?;
        let prices: Vec<PriceCollection> = cursor.try_collect().await?;
        Ok(prices)
    }
}
