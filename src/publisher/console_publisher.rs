use async_trait::async_trait;

use crate::domain::event::Event;
use crate::utils::serialize::to_json;

use super::Publisher;

pub struct ConsolePublisher;

#[async_trait]
impl Publisher for ConsolePublisher {
    async fn publish(&self, event: Box<dyn Event + Send + Sync>) -> anyhow::Result<()> {
        let json_str = to_json(event.as_ref())?;
        println!("{}", json_str);
        Ok(())
    }
}
