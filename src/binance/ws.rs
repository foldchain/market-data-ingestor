use futures::StreamExt;
use tokio::sync::broadcast::Sender;
use tokio_tungstenite::connect_async;
use url::Url;

use crate::domain::market_event::MarketEvent;
use anyhow::Result;

pub async fn connect_binance(symbol: &str, tx: Sender<MarketEvent>) -> Result<()> {
    let stream = format!("{}@trade", symbol.to_lowercase());
    let url = Url::parse(&format!("wss://stream.binance.com:9443/ws/{}", stream))?;

    let (ws_stream, _) = connect_async(url.as_str()).await?;
    let (_, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        if let Ok(msg) = msg {
            if msg.is_text() {
                let raw = msg.to_text()?;
                let parsed: serde_json::Value = serde_json::from_str(raw)?;
                let event = MarketEvent::Trade {
                    symbol: parsed["s"].as_str().unwrap_or_default().to_string(),
                    price: parsed["p"].as_str().unwrap_or("0.0").parse()?,
                    quantity: parsed["q"].as_str().unwrap_or("0.0").parse()?,
                    timestamp: parsed["T"].as_u64().unwrap_or(0),
                };
                let _ = tx.send(event);
            }
        }
    }

    Ok(())
}
