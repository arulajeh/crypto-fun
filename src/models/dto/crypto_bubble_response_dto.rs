use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoDataDto {
    pub id: Option<String>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub symbol: Option<String>,
    pub dominance: Option<f64>,
    pub image: Option<String>,
    pub rank: u32,
    pub stable: bool,
    pub price: Option<f64>,
    pub marketcap: Option<u64>,
    pub volume: Option<u64>,
    pub cg_id: Option<String>,
    pub symbols: Symbols,
    pub performance: Performance,
    #[serde(rename = "rankDiffs")]
    pub rank_diffs: RankDiffs,
    #[serde(rename = "exchangePrices")]
    pub exchange_prices: ExchangePrices,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Performance {
    pub min15: Option<f64>,
    pub hour4: Option<f64>,
    pub min5: Option<f64>,
    pub hour: Option<f64>,
    pub month3: Option<f64>,
    pub week: Option<f64>,
    pub year: Option<f64>,
    pub month: Option<f64>,
    pub min1: Option<f64>,
    pub day: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RankDiffs {
    pub min5: Option<i32>,
    pub min15: Option<i32>,
    pub day: Option<i32>,
    pub hour4: Option<i32>,
    pub month3: Option<i32>,
    pub hour: Option<i32>,
    pub week: Option<i32>,
    pub min1: Option<i32>,
    pub month: Option<i32>,
    pub year: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Symbols {
    pub binance: Option<String>,
    pub bingx: Option<String>,
    pub bitget: Option<String>,
    pub bitmart: Option<String>,
    pub bybit: Option<String>,
    pub coinbase: Option<String>,
    pub cryptocom: Option<String>,
    pub gateio: Option<String>,
    pub kraken: Option<String>,
    pub kucoin: Option<String>,
    pub mexc: Option<String>,
    pub okx: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangePrices {
    pub binance: Option<f64>,
    pub bingx: Option<f64>,
    pub bitget: Option<f64>,
    pub bitmart: Option<f64>,
    pub bybit: Option<f64>,
    pub coinbase: Option<f64>,
    pub cryptocom: Option<f64>,
    pub gateio: Option<f64>,
    pub kraken: Option<f64>,
    pub kucoin: Option<f64>,
    pub mexc: Option<f64>,
    pub okx: Option<f64>,
}