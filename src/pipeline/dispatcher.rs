use crate::domain::market_event::MarketEvent;
use tokio::sync::broadcast;

pub fn create_channel() -> broadcast::Sender<MarketEvent> {
    broadcast::channel(1024).0
}
