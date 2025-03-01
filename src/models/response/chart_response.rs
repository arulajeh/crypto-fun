use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChartResponse {
    pub interval: String,
    pub data: Vec<ChartTicks>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChartTicks {
    pub t: Option<u64>,
    pub p: Option<f64>,
}
