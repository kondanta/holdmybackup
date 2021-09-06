use {
    holdmybackup::config::{
        args::Opt,
        config_file::Config,
    },
    holdmybackup::http,
    holdmybackup::log,
    holdmybackup::storage::minio::MinioStore,
    std::sync::{
        Arc,
        Mutex,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Arc::new(Mutex::new(
        Config::load_config().expect("Cannot parse the config"),
    ));
    let args = Opt::args();
    let cloned_config = Arc::clone(&cfg);

    match Config::watch_config_changes(cloned_config) {
        Ok(()) => tracing::debug!("Config loaded"),
        Err(e) => tracing::error!("Cannot reload config: {:#?}", e),
    };

    match log::init_tracer(cfg.clone()) {
        Ok(()) => tracing::debug!("Tracer initialized."),
        Err(e) => tracing::error!("Cannot init tracer: {:#?}", e),
    };

    let http_server = {
        let addr = args.address.parse().unwrap_or_else(|_| {
            tracing::error!(
                "Cannot parse the http address string. Using the default value",
            );
            std::net::SocketAddr::from(([127, 0, 0, 1], 9090))
        });
        let cfg = cfg.clone();
        let service = hyper::service::make_service_fn(move |_| {
            let cfg = cfg.clone();
            async move {
                Ok::<_, hyper::Error>(hyper::service::service_fn(move |req| {
                    let cfg = cfg.clone();
                    let store = match MinioStore::init(cfg.clone()) {
                        Ok(s) => s,
                        Err(e) => {
                            anyhow::anyhow!(
                                "Cannot init object store: {:#?}",
                                e
                            );
                            std::process::exit(1);
                        }
                    };
                    http::router(req, cfg, store)
                }))
            }
        });
        hyper::Server::bind(&addr).serve(service)
    };

    http_server
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("failed to start http server");

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install ctrl+c signal handler");
}
