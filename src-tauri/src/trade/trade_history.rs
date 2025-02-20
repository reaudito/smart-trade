use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Trade {
    pub action: String,
    pub symbol: String,
    pub price: f64,
    pub timestamp: i64,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct TradeSwap {
    pub pool_id: u64, 
    pub token_in: String, 
    pub amount_in: String, 
    pub token_out: String,
    pub timestamp: i64,
}


