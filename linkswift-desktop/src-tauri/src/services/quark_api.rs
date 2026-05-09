use crate::error::AppError;
use crate::models::file::{FileItem, PaginatedFiles};
use crate::models::share::{ShareInfo, ShareTokenResponse};
use crate::models::task::DownloadLink;

pub struct QuarkApi {
    base_url: String,
    cookie: String,
}

impl QuarkApi {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            cookie: String::new(),
        }
    }

    pub fn with_cookie(mut self, cookie: &str) -> Self {
        self.cookie = cookie.to_string();
        self
    }

    pub fn parse_share_url(url: &str) -> Result<String, AppError> {
        todo!()
    }

    pub async fn get_share_token(&self, pwd_id: &str, passcode: Option<&str>) -> Result<String, AppError> {
        todo!()
    }

    pub async fn get_share_files(&self, pwd_id: &str, stoken: &str, dir_fid: &str, page: u32, size: u32) -> Result<PaginatedFiles, AppError> {
        todo!()
    }

    pub async fn transfer_files(
        &self,
        pwd_id: &str,
        stoken: &str,
        fid_list: &[String],
        fid_token_list: &[String],
        target_dir_fid: &str,
    ) -> Result<String, AppError> {
        todo!()
    }

    pub async fn query_transfer_task(&self, task_id: &str) -> Result<TransferTaskStatus, AppError> {
        todo!()
    }

    pub async fn get_download_link(&self, fid: &str) -> Result<DownloadLink, AppError> {
        todo!()
    }

    pub async fn verify_credential(&self) -> Result<bool, AppError> {
        todo!()
    }

    pub async fn get_user_directories(&self, parent_fid: &str) -> Result<Vec<FileItem>, AppError> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransferTaskStatus {
    Pending,
    Running { progress: u32 },
    Completed { new_fids: Vec<String> },
    Failed { reason: String },
}
