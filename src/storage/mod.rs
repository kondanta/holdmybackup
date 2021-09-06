pub mod minio;

use async_trait::async_trait;

#[async_trait]
pub trait ObjectStorage {
    async fn upload(&self) -> anyhow::Result<()>;
    async fn create_bucket(&self) -> anyhow::Result<bool>;g
}
