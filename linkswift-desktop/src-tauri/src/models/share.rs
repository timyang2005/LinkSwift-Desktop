use serde::{Deserialize, Serialize};
use crate::models::file::FileItem;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ShareInfo {
    pub pwd_id: String,
    pub stoken: String,
    pub title: String,
    pub has_password: bool,
    pub files: Vec<FileItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ShareTokenRequest {
    pub pwd_id: String,
    pub passcode: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ShareTokenResponse {
    pub stoken: String,
}
