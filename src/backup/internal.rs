use {
    crate::config::config_file::Config,
    anyhow::Result,
    chrono::prelude::*,
    flate2::{
        write::GzEncoder,
        Compression,
    },
    std::fs::File,
    std::sync::{
        Arc,
        Mutex,
    },
};

pub struct Backup(pub Arc<Mutex<Config>>);

impl Backup {
    fn tar_name(
        &self,
        s: &str,
    ) -> Result<(String, String)> {
        tracing::trace!("Processing Path: {}", s);
        let folder_name = s.split('/').last().unwrap_or("");
        tracing::trace!("Processed folder_name: {}", folder_name);
        if folder_name.is_empty() {
            tracing::error!(
                path = &s,
                folder_name = folder_name,
                "Cannot extract folder name from given path.",
            );
            return Err(anyhow::anyhow!("File is empty!"));
        }
        let tar_name =
            format!("{}-{}.tar.gz", folder_name, Utc::now().format("%Y-%m-%d"));
        tracing::trace!(
            "Name of the tarfile: {}, Name of the folder: {}",
            tar_name,
            folder_name
        );
        Ok((tar_name, folder_name.to_string()))
    }

    #[tracing::instrument(skip(self))]
    pub fn create_tarball(&self) -> Result<()> {
        let paths = self.0.lock().unwrap().backup.backup_path.clone();
        tracing::trace!("List of backup canditates: {:?}", paths);
        for path in paths.iter() {
            tracing::trace!("Currently processing: {}", path);
            tracing::info!("Starting to create tar file for {}", &path);
            let (tar_name, folder_name) = self.tar_name(path)?;
            let tar_file = File::create(tar_name)?;
            let enc = GzEncoder::new(tar_file, Compression::fast());
            let mut tar = tar::Builder::new(enc);
            tracing::info!("Appending folder content into .tar.gz file");
            let append = tar.append_dir_all(folder_name, &path);
            match append {
                Ok(_) => tracing::trace!("Tar file created properly"),
                Err(e) => {
                    tracing::error!("Cannot fill tar file's content! {:?}", e)
                }
            };
        }
        Ok(())
    }

    pub fn delete_tar_file(&self) -> Result<()> {
        let paths = self.0.lock().unwrap().backup.backup_path.clone();
        for f in paths {
            let (tar_name, _) = self.tar_name(&f)?;
            std::fs::remove_file(tar_name)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::config_file::Config;
    use std::sync::{
        Arc,
        Mutex,
    };

    #[test]
    fn check_create_tarball() -> anyhow::Result<()> {
        let config = Arc::new(Mutex::new(Config::load_config()?));
        let backup = Backup(config);

        backup.create_tarball().ok();

        Ok(())
    }
}
