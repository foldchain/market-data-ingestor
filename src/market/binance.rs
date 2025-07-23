use async_trait::async_trait;
use futures::StreamExt;
use std::sync::Arc;

use tokio_tungstenite::connect_async;
use url::Url;

use crate::domain::trade_event::TradeEvent;
use crate::market::Market;
use crate::publisher::Publisher;

pub struct BinanceMarket;

#[async_trait]
impl Market for BinanceMarket {
    async fn connect(symbol: &str, publisher: Arc<dyn Publisher>) -> anyhow::Result<()> {
        let stream = format!("{}@trade", symbol.to_lowercase());
        let url = Url::parse(&format!("wss://stream.binance.com:9443/ws/{}", stream))?;

        let (ws_stream, _) = connect_async(url.as_str()).await?;
        let (_, mut read) = ws_stream.split();

        while let Some(msg) = read.next().await {
            if let Ok(msg) = msg
                && msg.is_text()
            {
                let raw = msg.to_text()?;
                let parsed: serde_json::Value = serde_json::from_str(raw)?;
                let event = TradeEvent {
                    symbol: parsed["s"].as_str().unwrap_or_default().to_string(),
                    price: parsed["p"].as_str().unwrap_or("0.0").parse()?,
                    quantity: parsed["q"].as_str().unwrap_or("0.0").parse()?,
                    timestamp: parsed["T"].as_u64().unwrap_or(0),
                };
                let _ = publisher.publish(Box::from(event)).await?;
            }
        }

        Ok(())
    }
}
