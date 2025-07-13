mod binance;
mod domain;
mod pipeline;

use crate::binance::ws::connect_binance;
use crate::pipeline::dispatcher::create_channel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let tx = create_channel();

    let handle = tokio::spawn(async move {
        if let Err(e) = connect_binance("btcusdt", tx).await {
            eprintln!("Error: {:?}", e);
        }
    });

    handle.await?;
    Ok(())
}

