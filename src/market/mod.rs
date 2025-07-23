pub mod binance;

use std::sync::Arc;

use async_trait::async_trait;

use crate::publisher::Publisher;

#[async_trait]
pub trait Market {
    async fn connect(symbol: &str, publisher: Arc<dyn Publisher>) -> anyhow::Result<()>;
}
