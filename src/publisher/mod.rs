pub mod console_publisher;

use crate::domain::event::Event;
use async_trait::async_trait;

#[async_trait]
pub trait Publisher: Send + Sync {
    async fn publish(&self, event: Box<dyn Event + Send + Sync>) -> anyhow::Result<()>;
}
