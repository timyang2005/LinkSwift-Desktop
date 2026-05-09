use serde::{Deserialize, Serialize};
use crate::models::file::FileItem;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DownloadTask {
    pub id: String,
    pub share_url: String,
    pub files: Vec<FileItem>,
    pub status: TaskStatus,
    pub target_dir: String,
    pub rpc_server_id: String,
    pub created_at: i64,
    pub error_message: Option<String>,
    pub retry_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskStatus {
    Pending,
    Parsing,
    Transferring { progress: u32 },
    FetchingLink,
    Pushing,
    Completed,
    Failed { reason: String },
    Cancelled,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DownloadLink {
    pub fid: String,
    pub name: String,
    pub url: String,
    pub size: u64,
    pub md5: Option<String>,
    pub expires_in: u32,
}
