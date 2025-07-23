use std::sync::Arc;

use crate::market::Market;
use crate::market::binance::BinanceMarket;
use crate::publisher::console_publisher::ConsolePublisher;

mod domain;
mod market;
mod publisher;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let tx = ConsolePublisher;

    let handle = tokio::spawn(async move {
        if let Err(e) = BinanceMarket::connect("btcusdt", Arc::new(tx)).await {
            eprintln!("Error: {:?}", e);
        }
    });

    handle.await?;
    Ok(())
}
