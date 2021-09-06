use {
    crate::backup::interface::BackupInterface,
    crate::config::config_file::Config,
    hyper::{
        Body,
        Response,
    },
    serde::Serialize,
    std::sync::{
        Arc,
        Mutex,
    },
};

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
