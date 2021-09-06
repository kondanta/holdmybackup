use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) list: Option<Vec<String>>,
}
