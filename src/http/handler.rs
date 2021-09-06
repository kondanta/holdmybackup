use {
    crate::backup::Backup,
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

pub(super) async fn create_tarball(
    cfg: Arc<Mutex<Config>>
) -> anyhow::Result<Response<Body>> {
    let backup = Backup(cfg);
    let mut s = "Cannot create tarball";
    let r = backup.create_tarball();
    if r.is_ok() {
        s = "Tarball Creation's started."
    }
    Ok(Response::new(Body::from(s)))
}

pub(super) async fn delete_tar_files(
    cfg: Arc<Mutex<Config>>
) -> anyhow::Result<Response<Body>> {
    let backup = Backup(cfg);
    let mut s = "Cannot delete tarballs";
    let r = backup.delete_tar_file();
    if r.is_ok() {
        s = "Tarball deletion's started.";
    }
    Ok(Response::new(Body::from(s)))
}

pub(super) async fn upload_backup(
    _req: Request<Body>,
    bucket: impl crate::storage::ObjectStorage,
) -> anyhow::Result<Response<Body>> {
    // Bucket should be initialized before executing this function.
    // So, passing it as an argument into this function?
    let result = bucket.upload().await;
    let mut s = "No backup";
    if result.is_ok() {
        s = "Backup operation has triggered!";
    }
    Ok(Response::new(Body::from(s)))
}
