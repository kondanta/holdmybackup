use {
    serde::Deserialize,
    std::{
        fs::File,
        io::BufReader,
    },
    structopt::StructOpt,
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
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Hold My Backup")]
pub struct Opt {
    /// HTTP Server address
    #[structopt(long, default_value = "127.0.0.1:9090")]
    pub address: String,
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
    "#;

        let d: Config = serde_yaml::from_str(&yaml).unwrap();
        println!("{:#?}", d);
    }
}
