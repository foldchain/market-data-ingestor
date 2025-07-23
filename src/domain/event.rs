use erased_serde::Serialize;

pub trait Event: Serialize + Send + Sync {
    fn kind(&self) -> &'static str;
}
