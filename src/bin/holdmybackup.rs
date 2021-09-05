use notify::event::AccessKind;

use {
    holdmybackup::config::Config,
    hyper::{
        Body,
        Method,
        Request,
        Response,
        StatusCode,
    },
    notify::{
        event::AccessMode,
        Error,
        Event,
        RecursiveMode,
        Watcher,
    },
    std::{
        path::Path,
        sync::{
            Arc,
            Mutex,
        },
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Arc::new(Mutex::new(
        Config::load_config().expect("Cannot parse the config"),
    ));
    let cloned_config = Arc::clone(&cfg);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 9090));

    let mut watcher = notify::recommended_watcher(
        move |res: Result<Event, Error>| match res {
            Ok(e) => {
                // TODO(taylan): Print event with debug.
                // println!("Event: {:#?}", &e);
                if e.kind ==
                    notify::EventKind::Access(AccessKind::Close(
                        AccessMode::Write,
                    ))
                {
                    match Config::load_config() {
                        Ok(new_config) => {
                            // TODO(taylan): Print this event with
                            // println!("{:#?}", &new_config);
                            *cloned_config
                                .lock()
                                .expect("Cannot acquire the lock.") = new_config
                        }
                        Err(e) => println!("Error reloading config: {:#?}", e),
                    }
                }
            }
            // TODO(taylan): Print this error with log::error. Or
            // tracing::error!.
            Err(e) => println!("Error loading config: {:#?}", e),
        },
    )?;
    watcher.configure(notify::Config::NoticeEvents(true))?;
    watcher
        .watch(Path::new("config.yaml"), RecursiveMode::Recursive)
        .map_err(|e| anyhow::anyhow!("Cannot listen to file {:#?}", e))?;

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
