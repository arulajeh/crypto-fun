use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartResponseDto {
    pub t: Option<u64>,
    pub p: Option<f64>,
}