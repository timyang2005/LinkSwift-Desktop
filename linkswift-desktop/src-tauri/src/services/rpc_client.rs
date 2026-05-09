use crate::error::AppError;
use crate::models::config::{DownloaderType, RpcServer};

pub struct RpcClient {
    url: String,
    token: Option<String>,
    downloader_type: DownloaderType,
}

impl RpcClient {
    pub fn new(url: &str, token: Option<&str>) -> Self {
        Self {
            url: url.to_string(),
            token: token.map(|s| s.to_string()),
            downloader_type: DownloaderType::Aria2,
        }
    }

    pub fn with_downloader_type(mut self, dt: DownloaderType) -> Self {
        self.downloader_type = dt;
        self
    }

    pub async fn add_uri(&self, urls: Vec<&str>, filename: &str, dir: Option<&str>) -> Result<String, AppError> {
        todo!()
    }

    pub async fn test_connection(&self) -> Result<bool, AppError> {
        todo!()
    }

    pub async fn query_task_status(&self, task_id: &str) -> Result<RpcTaskStatus, AppError> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RpcTaskStatus {
    Active,
    Waiting,
    Paused,
    Error { message: String },
    Complete,
    Removed,
}
