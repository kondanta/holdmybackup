use {
    holdmybackup::config::config_file::Config,
    holdmybackup::log,
    hyper::{
        Body,
        Method,
        Request,
        Response,
        StatusCode,
    },
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
    let cloned_config = Arc::clone(&cfg);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 9090));

    match Config::watch_config_changes(cloned_config) {
        Ok(()) => tracing::debug!("Config loaded"),
        Err(e) => tracing::error!("Cannot reload config: {:#?}", e),
    };

    match log::init_tracer(cfg.clone()) {
        Ok(()) => tracing::debug!("Tracer initialized."),
        Err(e) => tracing::error!("Cannot init tracer: {:#?}", e),
    };

    let http_server = {
        let cfg = cfg.clone();
        let service = hyper::service::make_service_fn(move |_| {
            let cfg = cfg.clone();
            async move {
                Ok::<_, hyper::Error>(hyper::service::service_fn(move |req| {
                    let cfg = cfg.clone();
                    router(req, cfg)
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

async fn router(
    req: Request<Body>,
    cfg: Arc<Mutex<Config>>,
) -> anyhow::Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => show_config(req, cfg).await,
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

async fn show_config(
    _req: Request<Body>,
    cfg: Arc<Mutex<Config>>,
) -> anyhow::Result<Response<Body>> {
    let d = cfg.lock().unwrap().storage.backup_path.clone();
    Ok(Response::new(Body::from(d)))
}
