use {
    anyhow::Result,
    crossbeam_channel,
    notify::{
        event::{
            AccessKind,
            AccessMode,
        },
        RecommendedWatcher,
        RecursiveMode,
        Watcher,
    },
    serde::Deserialize,
    std::{
        fs::File,
        io::BufReader,
        path::Path,
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
    pub backup:    Backup,
    pub storage:   Storage,
    pub verbosity: String,
}

impl Config {
    pub fn load_config() -> Result<Config, anyhow::Error> {
        let f = BufReader::new(File::open("./config.yaml")?);
        serde_yaml::from_reader(f).map_err(|e| {
            anyhow::anyhow!("Cannot parse the config file: {}", e.to_string())
        })
    }

    pub fn watch_config_changes(cfg: Arc<Mutex<Config>>) -> Result<()> {
        std::thread::spawn(move || loop {
            let (tx, rx) = crossbeam_channel::unbounded();

            let mut watcher: RecommendedWatcher = RecommendedWatcher::new(tx)
                .map_err(|e| anyhow::anyhow!("Cannot create watcher: {:#?}", e))
                .expect("Cannot create Watcher");

            watcher
                .watch(Path::new("config.yaml"), RecursiveMode::NonRecursive)
                .expect("Cannot listen configuration file. Exiting...");

            match rx.recv().map_err(|e| {
                anyhow::anyhow!("Receiving error on config watcher: {:#?}", e)
            }) {
                Ok(event) => match event {
                    Ok(e) => {
                        if e.kind ==
                            notify::EventKind::Access(AccessKind::Close(
                                AccessMode::Write,
                            ))
                        {
                            match Config::load_config() {
                                Ok(new_config) => {
                                    *cfg.lock().unwrap() = new_config
                                }
                                Err(e) => {
                                    tracing::error!(
                                        "Error reloading config: {:#?}",
                                        e
                                    )
                                }
                            }
                        }
                    }
                    Err(e) => tracing::error!("Event error: {:#?}", e),
                },
                Err(e) => tracing::error!("Watch error: {:#?}", e.to_string()),
            }
        });
        Ok(())
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
