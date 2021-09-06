use {
    crate::config::config_file::Config,
    hyper::{
        Body,
        Request,
        Response,
    },
    std::sync::{
        Arc,
        Mutex,
    },
};
pub(super) async fn show_config(
    _req: Request<Body>,
    cfg: Arc<Mutex<Config>>,
) -> anyhow::Result<Response<Body>> {
    let d = cfg.lock().unwrap().storage.backup_path.clone();
    Ok(Response::new(Body::from(d)))
}
