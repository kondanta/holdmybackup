use {
    anyhow::Result,
    reload_config,
    serde::Deserialize,
    std::{
        fs::File,
        io::BufReader,
        sync::{
            Arc,
            Mutex,
        },
    },
};

#[derive(Debug, Deserialize, Clone)]
pub enum BackupStrategy {
    KeepEverything,
    KeepLastN(u32),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Backup {
    /// Defines how we should handle the backup.
    pub strategy:    BackupStrategy,
    /// List of the files that going to be backupped :)
    pub backup_path: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Minio {
    /// Access Key
    pub access_key: String,
    /// Secret Key
    pub secret_key: String,

    pub endpoint: String,

    pub region: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Storage {
    /// Backup path you would like to use. If it is empty,
    /// Service will create one for you.
    pub backup_path: String,
    /// Minio Config
    pub minio:       Option<Minio>,
    // TODO(taylan): Add AWS: S3 later.
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub backup:  Backup,
    pub storage: Storage,
}

impl Config {
    #[tracing::instrument]
    pub fn load_config() -> Result<Config> {
        tracing::info!("Loading config...");
        let opt = crate::config::args::Opt::args();
        let f = BufReader::new(File::open(opt.config_path)?);
        tracing::debug!("File content: {:?}", &f);
        let r = serde_yaml::from_reader(f).map_err(|e| {
            anyhow::anyhow!("Cannot parse the config file: {}", e.to_string())
        });
        tracing::trace!("Config content: {:#?}", &r);
        r
    }

    pub fn watch_config_changes(
        cfg: Arc<Mutex<Config>>,
        mode: String,
        config_path: String,
    ) -> Result<()> {
        tracing::info!("Creating config watcher");
        reload_config::watch_changes(
            cfg,
            mode,
            config_path,
            Config::load_config,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_yaml() {
        let yaml = r#"
storage:
    backup_path: "backups"
backup:
    strategy: "KeepEverything"
    backup_path: [ "." ]
verbosity: debug
    "#;

        let d: Config = serde_yaml::from_str(&yaml).unwrap();
        println!("{:#?}", d);
    }
}
