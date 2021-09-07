use {
    super::internal,
    crate::config::config_file::Config,
    crate::storage::minio::MinioStore,
    crate::storage::ObjectStorage,
    anyhow::Result,
    std::sync::{
        Arc,
        Mutex,
    },
};

#[derive(Debug)]
pub struct BackupInterface(pub Arc<Mutex<Config>>);

impl BackupInterface {
    pub async fn init(cfg: Arc<Mutex<Config>>) -> Self {
        Self(cfg)
    }

    #[tracing::instrument(skip(self))]
    pub async fn create(&self) -> Result<()> {
        let backup = internal::Backup(self.0.clone());
        backup.create_tarball().ok();
        let storage: MinioStore = ObjectStorage::init(self.0.clone())?;
        storage.upload().await.ok();
        backup.delete_tar_file().ok();
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn list(&self) -> Result<String> {
        let storage: MinioStore = ObjectStorage::init(self.0.clone())?;
        storage.list().await
    }
}
