use crate::{
    backup::interface::BackupInterface,
    config::config_file::Config,
    parse_body,
};
use anyhow::{
    anyhow,
    Result,
};
use hyper::{
    Body,
    Response,
    StatusCode,
};
use serde::Deserialize;
use std::sync::{
    Arc,
    Mutex,
};
use tokio::runtime::Handle;
use tracing::{
    debug,
    error,
};
use tracing_subscriber::{
    layer::Layered,
    EnvFilter,
    Registry,
};

pub type HandleType = tracing_subscriber::reload::Handle<
    EnvFilter,
    Layered<tracing_subscriber::fmt::Layer<Registry>, Registry>,
>;

#[derive(Debug, Deserialize)]
struct FilterRequest {
    filter: String,
}

pub(super) async fn create_backup(
    cfg: Arc<Mutex<Config>>
) -> Result<Response<Body>> {
    let runtime = Handle::current();
    let msg =
        serde_json::json!({"response": "Your backup request's been recorded."})
            .to_string();
    runtime.spawn(async move { do_create_backup(cfg).await });
    Ok(Response::new(Body::from(msg)))
}

async fn do_create_backup(cfg: Arc<Mutex<Config>>) -> Result<()> {
    let backup = BackupInterface::init(cfg).await;
    let r = backup.create().await;

    match r {
        Ok(_) => debug!("Backup created"),
        Err(e) => {
            error!("Cannot create the backup! {}", e.to_string())
        }
    };

    Ok(())
}

pub(super) async fn list_backups(
    cfg: Arc<Mutex<Config>>
) -> Result<Response<Body>> {
    let backup = BackupInterface::init(cfg).await;
    let data = backup.list().await?;
    let model: String = serde_json::json!({ "response": data }).to_string();
    Ok(Response::new(Body::from(model)))
}

pub(super) async fn filter(
    mut req: hyper::Request<Body>,
    _cfg: Arc<Mutex<Config>>,
    handle: HandleType,
) -> Result<Response<Body>> {
    let request_body = parse_body!(&mut req.body_mut());
    let deserialized_body: FilterRequest =
        serde_json::from_slice(&request_body)?;

    let new_filter = deserialized_body
        .filter
        .parse::<tracing_subscriber::filter::EnvFilter>()
        .map_err(|e| anyhow!("Parsing filter error: {:#?}", e))?;

    let h = handle
        .reload(new_filter)
        .map_err(|e| anyhow!("Reloading error: {:#?}", e));

    match h {
        Ok(_) => {
            debug!("Changed log level");
            Ok(Response::new(Body::from(
                serde_json::json!({"response": "OK"}).to_string(),
            )))
        }
        Err(e) => {
            let mut err = Response::new(Body::from(e.to_string()));
            *err.status_mut() = StatusCode::NOT_FOUND;
            Ok(err)
        }
    }
}

// pub(super) async fn server_info() -> Result<Response<Body>> {}
