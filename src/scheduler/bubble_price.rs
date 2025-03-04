use crate::database::collections::price_collection::PriceCollection;
use crate::models::dto::crypto_bubble_response_dto::{
    CryptoDataDto, ExchangePrices, Performance, RankDiffs, Symbols,
};
use crate::repositories::currency_repository::CurrencyRepository;
use crate::repositories::price_repository::PriceRepository;
use mongodb::Database;
use std::env;
use std::sync::Arc;

pub async fn fetch_bubble_price(db: Arc<Database>) {
    println!("Fetching bubble price...");
    let currency_repository = CurrencyRepository::new(&db);
    let currencies = match currency_repository.get_all_currency().await {
        Ok(data) => data,
        Err(_) => {
            println!("Failed to fetch currency data");
            return;
        }
    };

    let currency_names: Vec<String> = currencies.into_iter().map(|c| c.name).collect();
    let source_host = env::var("DATA_SOURCE_HOST").expect("DATA_SOURCE_HOST must be set");

    let price_repository = Arc::new(PriceRepository::new(&db));

    loop {
        let price_repo_clone = Arc::clone(&price_repository);
        fetch_data(&source_host, &currency_names, price_repo_clone).await;

        let interval = env::var("FETCH_INTERVAL").unwrap_or_else(|_| "60".to_string());
        let interval = interval.parse::<u64>().unwrap_or(60);
        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
    }
}

async fn fetch_data(host: &str, currency_names: &[String], price_repository: Arc<PriceRepository>) {
    let currencies = currency_names.to_owned();
    for currency_name in currencies {
        let host = host.to_string();
        let price_repository = Arc::clone(&price_repository);

        tokio::spawn(async move {
            let url = format!("{}/data/bubbles1000.{}.json", host, currency_name.to_lowercase());
            match reqwest::get(url).await {
                Ok(response) => match response.json::<Vec<CryptoDataDto>>().await {
                    Ok(data) => {
                        for crypto_price in data {

                            let image_name = match crypto_price.image.clone() {
                                Some(image) => image.split("/").last().unwrap().to_string(),
                                None => "".to_string(),
                            };

                            let image = format!("logo/{}", image_name);
                            let price = PriceCollection {
                                object_id: None,
                                id: crypto_price.id,
                                name: crypto_price.name,
                                slug: crypto_price.slug,
                                symbol: crypto_price.symbol,
                                dominance: crypto_price.dominance,
                                image: Option::from(image),
                                rank: crypto_price.rank,
                                stable: crypto_price.stable,
                                price: crypto_price.price,
                                marketcap: crypto_price.marketcap,
                                volume: crypto_price.volume,
                                cg_id: crypto_price.cg_id,
                                symbols: Symbols {
                                    binance: crypto_price.symbols.binance,
                                    bingx: crypto_price.symbols.bingx,
                                    bitget: crypto_price.symbols.bitget,
                                    bitmart: crypto_price.symbols.bitmart,
                                    bybit: crypto_price.symbols.bybit,
                                    coinbase: crypto_price.symbols.coinbase,
                                    cryptocom: crypto_price.symbols.cryptocom,
                                    gateio: crypto_price.symbols.gateio,
                                    kraken: crypto_price.symbols.kraken,
                                    kucoin: crypto_price.symbols.kucoin,
                                    mexc: crypto_price.symbols.mexc,
                                    okx: crypto_price.symbols.okx,
                                },
                                performance: Performance {
                                    min15: crypto_price.performance.min15,
                                    hour4: crypto_price.performance.hour4,
                                    min5: crypto_price.performance.min5,
                                    hour: crypto_price.performance.hour,
                                    month3: crypto_price.performance.month3,
                                    week: crypto_price.performance.week,
                                    year: crypto_price.performance.year,
                                    month: crypto_price.performance.month,
                                    min1: crypto_price.performance.min1,
                                    day: crypto_price.performance.day,
                                },
                                currency: currency_name.clone(),
                                rank_diffs: RankDiffs {
                                    min5: crypto_price.rank_diffs.min5,
                                    min15: crypto_price.rank_diffs.min15,
                                    day: crypto_price.rank_diffs.day,
                                    hour4: crypto_price.rank_diffs.hour4,
                                    month3: crypto_price.rank_diffs.month3,
                                    hour: crypto_price.rank_diffs.hour,
                                    week: crypto_price.rank_diffs.week,
                                    min1: crypto_price.rank_diffs.min1,
                                    month: crypto_price.rank_diffs.month,
                                    year: crypto_price.rank_diffs.year,
                                },
                                exchange_prices: ExchangePrices {
                                    binance: crypto_price.exchange_prices.binance,
                                    bingx: crypto_price.exchange_prices.bingx,
                                    bitget: crypto_price.exchange_prices.bitget,
                                    bitmart: crypto_price.exchange_prices.bitmart,
                                    bybit: crypto_price.exchange_prices.bybit,
                                    coinbase: crypto_price.exchange_prices.coinbase,
                                    cryptocom: crypto_price.exchange_prices.cryptocom,
                                    gateio: crypto_price.exchange_prices.gateio,
                                    kraken: crypto_price.exchange_prices.kraken,
                                    kucoin: crypto_price.exchange_prices.kucoin,
                                    mexc: crypto_price.exchange_prices.mexc,
                                    okx: crypto_price.exchange_prices.okx,
                                },
                            };
                            match price_repository.save_or_update(price).await {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Error: {:?}", e);
                                    println!("Failed to fetch data for {}", currency_name);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        println!("Failed to fetch data for {}", currency_name);
                    }
                },
                Err(_) => {
                    println!("Failed to fetch data for {}", currency_name);
                }
            }
        });
    }
}
