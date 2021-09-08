use {
    crate::backup::interface::BackupInterface,
    crate::config::config_file::Config,
    crate::parse_body,
    hyper::{
        Body,
        Response,
        StatusCode,
    },
    serde::{
        Deserialize,
        Serialize,
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

#[derive(Debug, Serialize)]
struct JsonResponse<'a> {
    response: &'a str,
}

impl<'a> JsonResponse<'a> {
    fn default() -> JsonResponse<'a> {
        JsonResponse {
            response: "Default Json Response",
        }
    }

    fn set_msg(
        &mut self,
        s: &'a str,
    ) -> &Self {
        self.response = s;
        self
    }
}

#[derive(Debug, Deserialize)]
struct FilterRequest {
    filter: String,
}

pub(super) async fn create_backup(
    cfg: Arc<Mutex<Config>>
) -> anyhow::Result<Response<Body>> {
    let backup = BackupInterface::init(cfg).await;
    let mut msg = JsonResponse::default();
    let r = backup.create().await;
    if r.is_ok() {
        msg.set_msg("Backup created");
    }

    let s = serde_json::to_string(&msg)?;
    Ok(Response::new(Body::from(s)))
}

pub(super) async fn list_backups(
    cfg: Arc<Mutex<Config>>
) -> anyhow::Result<Response<Body>> {
    let backup = BackupInterface::init(cfg).await;
    let msg = backup.list().await?;
    Ok(Response::new(Body::from(msg)))
}

pub(super) async fn filter(
    mut req: hyper::Request<Body>,
    _cfg: Arc<Mutex<Config>>,
    handle: HandleType,
) -> anyhow::Result<Response<Body>> {
    let request_body = parse_body!(&mut req.body_mut());
    let deserialized_body: FilterRequest =
        serde_json::from_slice(&request_body)?;
    let new_filter = deserialized_body
        .filter
        .parse::<tracing_subscriber::filter::EnvFilter>()
        .map_err(|e| anyhow::anyhow!("Parsing filter error: {:#?}", e))?;
    let h = handle
        .reload(new_filter)
        .map_err(|e| anyhow::anyhow!("Reloading error: {:#?}", e));

    match h {
        Ok(_) => Ok(Response::new(Body::from(""))),
        Err(e) => {
            let mut err = Response::new(Body::from(e.to_string()));
            *err.status_mut() = StatusCode::NOT_FOUND;
            Ok(err)
        }
    }
}
