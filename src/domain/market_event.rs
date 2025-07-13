use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketEvent {
    Trade {
        symbol: String,
        price: f64,
        quantity: f64,
        timestamp: u64,
    },
    OrderBookUpdate {
        symbol: String,
        bids: Vec<(f64, f64)>,
        asks: Vec<(f64, f64)>,
        timestamp: u64,
    },
}
