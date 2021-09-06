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

pub(super) async fn perform_backup(
    _req: Request<Body>,
    bucket: impl crate::storage::ObjectStorage,
) -> anyhow::Result<Response<Body>> {
    // Bucket should be initialized before executing this function.
    // So, passing it as an argument into this function?
    let result = bucket.upload().await?;
    let mut s = "No backup";
    if result == () {
        s = "Backup operation has triggered!";
    }
    Ok(Response::new(Body::from(s)))
}
