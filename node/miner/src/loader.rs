use async_trait::async_trait;
use std::sync::Arc;
use storage::log_store::{MineLoadChunk, Store};


#[async_trait]
pub trait PoraLoader: Send + Sync {
    async fn load_sealed_data(&self, index: u64) -> Option<MineLoadChunk>;
}

#[async_trait]
impl PoraLoader for Arc<dyn Store> {
    async fn load_sealed_data(&self, chunk_index: u64) -> Option<MineLoadChunk> {
        match self.flow().load_sealed_data(chunk_index) {
            Ok(Some(chunk)) => Some(chunk),
            _ => None,
        }
    }
}
