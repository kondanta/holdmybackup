use {
    crate::config::config_file::Config,
    anyhow::Result,
    std::str::FromStr,
    std::sync::{
        Arc,
        Mutex,
    },
    tracing::{
        subscriber,
        Level,
    },
    tracing_subscriber::filter::{
        Directive,
        EnvFilter,
    },
    tracing_subscriber::FmtSubscriber,
};

pub fn init_tracer(cfg: Arc<Mutex<Config>>) -> Result<()> {
    let filter = EnvFilter::from_default_env().add_directive(Directive::from(
        Level::from_str(cfg.lock().unwrap().verbosity.as_str())?,
    ));

    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    subscriber::set_global_default(subscriber).map_err(|e| {
        anyhow::anyhow!("Cannot set global default for tracer: {:#?}", e)
    })
}
