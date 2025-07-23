use crate::domain::event::Event;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeEvent {
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub timestamp: u64,
}

impl Event for TradeEvent {
    fn kind(&self) -> &'static str {
        "trade_event"
    }
}
