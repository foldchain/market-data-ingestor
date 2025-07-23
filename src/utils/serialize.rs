use serde_json::Serializer;

use crate::domain::event::Event;

pub fn to_json(event: &dyn Event) -> anyhow::Result<String> {
    let mut vec = Vec::new();
    {
        let mut serializer = Serializer::new(&mut vec);
        erased_serde::serialize(event, &mut serializer)?;
    }
    Ok(String::from_utf8(vec)?)
}
