use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Hold My Backup")]
pub struct Opt {
    /// HTTP Server address
    #[structopt(long, default_value = "127.0.0.1:9090")]
    pub address: String,
}
