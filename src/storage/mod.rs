pub mod minio;

use async_trait::async_trait;
use std::sync::{
    Arc,
    Mutex,
};

use crate::config::config_file::Config;

#[async_trait]
pub trait ObjectStorage {
    fn init(cfg: Arc<Mutex<Config>>) -> anyhow::Result<Self>
    where
        Self: Sized;
    async fn upload(&self) -> anyhow::Result<()>;
    async fn create_bucket(&self) -> anyhow::Result<bool>;

    async fn is_bucket_exists(&self) -> anyhow::Result<bool>;
}
