pub(super) mod handler;

use {
    super::config::config_file::Config,
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
    tracing_subscriber::layer::Layered,
    tracing_subscriber::EnvFilter,
    tracing_subscriber::Registry,
};

pub type HandleType = tracing_subscriber::reload::Handle<
    EnvFilter,
    Layered<tracing_subscriber::fmt::Layer<Registry>, Registry>,
>;
pub async fn router(
    req: Request<Body>,
    cfg: Arc<Mutex<Config>>,
    handle: HandleType,
) -> anyhow::Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/backup") => handler::create_backup(cfg).await,
        (&Method::GET, "/list") => handler::list_backups(cfg).await,
        (&Method::PUT, "/filter") => handler::filter(req, cfg, handle).await,
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[macro_export]
macro_rules! parse_body {
    ($req_body: expr) => {{
        use futures_util::StreamExt;

        let mut request_body = ::std::vec::Vec::new();
        while let Some(chunk) = StreamExt::next($req_body).await {
            request_body.extend_from_slice(&chunk.unwrap());
        }
        request_body
    }};
}
