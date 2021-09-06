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
};
pub async fn router(
    req: Request<Body>,
    cfg: Arc<Mutex<Config>>,
) -> anyhow::Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/backup") => handler::create_backup(cfg).await,
        (&Method::GET, "/list") => handler::list_backups(cfg).await,
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
