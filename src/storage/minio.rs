use {
    super::ObjectStorage,
    crate::config::config_file::Config,
    anyhow::Result,
    async_trait::async_trait,
    chrono::prelude::*,
    s3::bucket::Bucket,
    s3::creds::Credentials,
    s3::region::Region,
    s3::BucketConfiguration,
    std::sync::{
        Arc,
        Mutex,
    },
};

#[derive(Debug)]
pub struct MinioStore {
    bucket:       s3::bucket::Bucket,
    backup_paths: Vec<String>,
}

// TODO(taylan): Implementing Get and List will be useful when it comes to
// writing a client. Currently I am not bothered with implementing them because
// they won't add any value.
#[async_trait]
impl ObjectStorage for MinioStore {
    fn init(cfg: Arc<Mutex<Config>>) -> Result<Self> {
        let storage = cfg.lock().unwrap().storage.clone();
        let backup = cfg.lock().unwrap().backup.clone();
        match storage.minio {
            Some(m) => {
                let bucket = Bucket::new_with_path_style(
                    &storage.backup_path,
                    Region::Custom {
                        region:   m.region,
                        endpoint: m.endpoint,
                    },
                    Credentials {
                        access_key:     Some(m.access_key),
                        secret_key:     Some(m.secret_key),
                        security_token: None,
                        session_token:  None,
                    },
                )?;
                Ok(Self {
                    bucket,
                    backup_paths: backup.backup_path,
                })
            }
            None => Err(anyhow::anyhow!(
                "Trying to create minio without a configuration."
            )),
        }
    }

    async fn create_bucket(&self) -> Result<bool> {
        let is_exists = self.is_bucket_exists().await?;
        if !is_exists {
            let b = s3::bucket::Bucket::create_with_path_style(
                &self.bucket.name,
                self.bucket.region.clone(),
                self.bucket.credentials.clone(),
                BucketConfiguration::default(),
            )
            .await
            .map_err(|e| {
                tracing::error!("Cannot create bucket: {:#?}", e);
                anyhow::anyhow!("Cannot create bucket: {:#?}", e)
            });
            return Ok(b?.success());
        }
        Err(anyhow::anyhow!("Bucket already exists."))
    }

    #[tracing::instrument(skip(self))]
    async fn upload(&self) -> anyhow::Result<()> {
        for path in &self.backup_paths {
            let folder_name = path.split('/').last().unwrap_or("");
            let file_name = format!(
                "{}-{}.tar.gz",
                folder_name,
                Utc::now().format("%Y-%m-%d")
            );
            let full_path = format!("{}/{}", folder_name, file_name);
            tracing::info!(
                name = file_name.as_str(),
                path = full_path.as_str(),
                "Uploading file!",
            );
            let mut p = tokio::fs::File::open(&file_name).await?;
            let response =
                self.bucket.put_object_stream(&mut p, full_path).await;
            match response {
                Ok(r) => tracing::info!(
                    function = "put_object_stream",
                    "Put operation response: {}",
                    r
                ),
                Err(e) => {
                    tracing::error!(
                        "Cannot upload the object: {}",
                        e.to_string()
                    )
                }
            };
        }
        Ok(())
    }

    async fn is_bucket_exists(&self) -> Result<bool> {
        let (_, response_code) =
            self.bucket.head_object(self.bucket.name.as_str()).await?;
        if response_code == 404 {
            return Ok(false);
        }
        Ok(true)
    }

    async fn list(&self) -> Result<Vec<String>> {
        let result = self.bucket.list("/".to_string(), None).await?;
        let mut data: Vec<String> = Vec::new();
        for i in result {
            let r = i.contents;
            for d in r {
                data.push(d.key);
            }
        }
        Ok(data)
    }
}

#[cfg(test)]
mod cfg {
    use super::*;
    use crate::config::config_file::Config;
    use std::sync::{
        Arc,
        Mutex,
    };
    #[tokio::test]
    async fn create_bucket_if_not_exists() -> anyhow::Result<()> {
        let config = Arc::new(Mutex::new(Config::load_config()?));
        let bucket: MinioStore = MinioStore::init(config)?;
        let r = bucket.create_bucket().await.unwrap_or_else(|_| {
            println!("Cannot create bucket");
            false
        });
        // This will fail for the first time. Because /shrug
        assert_eq!(r, false);
        Ok(())
    }

    #[tokio::test]
    async fn upload_object() -> anyhow::Result<()> {
        let config = Arc::new(Mutex::new(Config::load_config()?));
        let bucket: MinioStore = MinioStore::init(config)?;

        let r = bucket.upload().await?;
        println!("{:#?}", r);

        Ok(())
    }

    #[tokio::test]
    async fn list() -> anyhow::Result<()> {
        let config = Arc::new(Mutex::new(Config::load_config()?));
        let bucket: MinioStore = MinioStore::init(config)?;
        let _ = bucket.list().await.unwrap();
        Ok(())
    }
}
