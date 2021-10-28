use {
    holdmybackup::config::{
        args::Opt,
        config_file::Config,
    },
    holdmybackup::http,
    holdmybackup::log,
    std::sync::{
        Arc,
        Mutex,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Opt::args();
    let config_path = args.config_path;
    let recursive_mode = args.recursive_mode;
    let log_level = args.log_level;
    let cfg = Arc::new(Mutex::new(
        Config::load_config().expect("Cannot parse the config"),
    ));
    let cloned_config = Arc::clone(&cfg);
    let otel_addr = cfg.lock().unwrap().otel_addr.clone();
    let handler = log::init_tracer(log_level, otel_addr)?;

    match Config::watch_config_changes(
        cloned_config,
        recursive_mode,
        config_path,
    ) {
        Ok(()) => tracing::debug!("Config loaded."),
        Err(e) => tracing::error!("Cannot reload config: {:#?}", e),
    };

    let http_server = {
        let addr = args.address.parse().unwrap_or_else(|_| {
            tracing::error!(
                "Cannot parse the http address string. Using the default value",
            );
            std::net::SocketAddr::from(([127, 0, 0, 1], 9090))
        });
        tracing::info!("Serving HTTP API through: {}", &addr);
        let handler = handler.clone();
        let cfg = cfg.clone();
        let service = hyper::service::make_service_fn(move |_| {
            let cfg = cfg.clone();
            let handler = handler.clone();
            async move {
                Ok::<_, hyper::Error>(hyper::service::service_fn(move |req| {
                    let cfg = cfg.clone();
                    let handler = handler.clone();
                    http::router(req, cfg, handler, addr)
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
