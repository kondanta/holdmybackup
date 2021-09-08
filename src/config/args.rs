use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Hold My Backup")]
pub struct Opt {
    /// HTTP Server address
    #[structopt(long, default_value = "127.0.0.1:9090")]
    pub address: String,

    /// Sets which path application should look for config file. Config file
    /// extension must be provided as well.
    #[structopt(short, long, default_value = "./config.yaml")]
    pub config_path: String,

    /// Sets whether config reload logic listens folder recursively, like
    /// editing sub-folder also triggers an event, or listen just the given
    /// path.
    #[structopt(long, default_value = "nonrecursive")]
    pub recursive_mode: String,

    /// Initial log level of the application
    #[structopt(short = "v", long = "verbosity", default_value = "info")]
    pub log_level: String,
}

impl Opt {
    pub fn args() -> Self
    where
        Self: Sized,
    {
        Opt::from_args()
    }
}
