use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Trade {
    pub action: String,
    pub symbol: String,
    pub price: f64,
    pub timestamp: i64,
}