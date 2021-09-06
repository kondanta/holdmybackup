pub(super) mod handler;

use {
    super::config::config_file::Config,
    super::storage::ObjectStorage,
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
pub async fn router(
    req: Request<Body>,
    cfg: Arc<Mutex<Config>>,
    storage: impl ObjectStorage,
) -> anyhow::Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => handler::show_config(req, cfg).await,
        (&Method::POST, "/upload") => {
            handler::upload_backup(req, storage).await
        }
        (&Method::POST, "/backup") => handler::create_tarball(cfg).await,
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
