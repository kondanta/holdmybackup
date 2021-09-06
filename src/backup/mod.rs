use {
    crate::config::config_file::Config,
    anyhow::Result,
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
        let folder_name = s.split('/').last().unwrap_or("");
        if folder_name.is_empty() {
            tracing::error!(
                path = &s,
                folder_name = folder_name,
                "Cannot extract folder name from given path.",
            );
            return Err(anyhow::anyhow!("File is empty!"));
        }
        let tar_name = format!("{}.tar.gz", folder_name);
        Ok((tar_name, folder_name.to_string()))
    }

    pub fn create_tarball(&self) -> Result<()> {
        let paths = self.0.lock().unwrap().backup.backup_path.clone();
        let mut backups: Vec<String> = Vec::new();
        for path in paths {
            let (tar_name, folder_name) = self.tar_name(&path)?;
            backups.push(tar_name.clone());
            let tar_file = File::create(tar_name)?;
            let enc = GzEncoder::new(tar_file, Compression::best());
            let mut tar = tar::Builder::new(enc);
            tar.append_dir_all(folder_name, &path)?;
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
