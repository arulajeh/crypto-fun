use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChartResponse {
    pub t: Option<u64>,
    pub p: Option<f64>,
}
