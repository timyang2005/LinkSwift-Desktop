//! Quark API Service 夸克网盘API服务
//!
//! 提供与夸克网盘API交互的功能封装

use crate::error::AppError;
use crate::models::file::{FileItem, PaginatedFiles};
use crate::models::task::DownloadLink;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize)]
pub enum TransferTaskStatus {
    Pending,
    Running { progress: u32 },
    Completed { new_fids: Vec<String> },
    Failed { reason: String },
}

#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    #[serde(default)]
    #[allow(dead_code)]
    code: Option<i32>,
    #[serde(default)]
    message: Option<String>,
    data: Option<T>,
}

#[derive(Debug, Deserialize)]
struct ShareTokenData {
    stoken: String,
}

#[derive(Debug, Deserialize)]
struct TaskIdData {
    task_id: String,
}

#[derive(Debug, Deserialize)]
struct TransferTaskData {
    #[serde(rename = "status")]
    status: u32,
    #[serde(rename = "progress", default)]
    progress: Option<u32>,
    #[serde(rename = "new_fids", default)]
    new_fids: Option<Vec<String>>,
    #[serde(rename = "failed_reason", default)]
    failed_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DownloadLinkData {
    fid: String,
    name: String,
    download_url: String,
    size: Option<u64>,
    md5: Option<String>,
    expires_in: Option<u64>,
}

pub struct QuarkApi {
    base_url: String,
    cookie: String,
    client: reqwest::Client,
}

impl QuarkApi {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            cookie: String::new(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_cookie(mut self, cookie: &str) -> Self {
        self.cookie = cookie.to_string();
        self
    }

    pub fn parse_share_url(url: &str) -> Result<String, AppError> {
        if url.is_empty() {
            return Err(AppError::ParseError("URL不能为空".to_string()));
        }

        let url_lower = url.to_lowercase();
        if !url_lower.starts_with("http://pan.quark.cn/s/")
            && !url_lower.starts_with("https://pan.quark.cn/s/")
        {
            return Err(AppError::ParseError("无效的分享链接域名".to_string()));
        }

        let parts: Vec<&str> = url.split("/s/").collect();
        if parts.len() < 2 {
            return Err(AppError::ParseError("URL格式不正确".to_string()));
        }

        let after_s = parts[1];
        let id_part = after_s
            .trim_end_matches('/')
            .split('?')
            .next()
            .unwrap_or("");

        if id_part.is_empty() {
            return Err(AppError::ParseError("分享ID不能为空".to_string()));
        }

        Ok(id_part.to_string())
    }

    pub async fn get_share_token(
        &self,
        pwd_id: &str,
        passcode: Option<&str>,
    ) -> Result<String, AppError> {
        let url = format!("{}/1/clouddrive/share/sharepage/token", self.base_url);

        let body = match passcode {
            Some(code) => json!({
                "pwd_id": pwd_id,
                "passcode": code
            }),
            None => json!({
                "pwd_id": pwd_id,
                "passcode": ""
            }),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        if status.as_u16() == 403 {
            return Err(AppError::ShareExpired("分享已失效".to_string()));
        }
        if status.as_u16() == 400 {
            return Err(AppError::ShareExpired("提取码错误".to_string()));
        }

        if !status.is_success() {
            return Err(AppError::NetworkError(format!("HTTP {}", status)));
        }

        let resp: ApiResponse<ShareTokenData> = response.json().await?;

        match resp.data {
            Some(data) => Ok(data.stoken),
            None => Err(AppError::NetworkError(resp.message.unwrap_or_default())),
        }
    }

    pub async fn get_share_files(
        &self,
        pwd_id: &str,
        stoken: &str,
        dir_fid: &str,
        page: u32,
        size: u32,
    ) -> Result<PaginatedFiles, AppError> {
        let url = format!(
            "{}/1/clouddrive/share/sharepage/detail?pwd_id={}&stoken={}&dir_fid={}&page={}&size={}",
            self.base_url, pwd_id, stoken, dir_fid, page, size
        );

        let response = self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.status().as_u16() == 401 {
            return Err(AppError::InvalidCookie);
        }

        if !response.status().is_success() {
            return Err(AppError::NetworkError(format!(
                "HTTP {}",
                response.status()
            )));
        }

        #[derive(Deserialize)]
        struct FilesData {
            list: Vec<FileItem>,
            total_count: u32,
            page: u32,
            page_size: u32,
            has_more: bool,
        }

        let resp: ApiResponse<FilesData> = response.json().await?;

        match resp.data {
            Some(data) => Ok(PaginatedFiles {
                items: data.list,
                total_count: data.total_count as u64,
                page: data.page,
                page_size: data.page_size,
                has_more: data.has_more,
            }),
            None => Err(AppError::NetworkError(resp.message.unwrap_or_default())),
        }
    }

    pub async fn transfer_files(
        &self,
        pwd_id: &str,
        stoken: &str,
        fid_list: &[String],
        fid_token_list: &[String],
        target_dir_fid: &str,
    ) -> Result<String, AppError> {
        if fid_list.is_empty() {
            return Err(AppError::TransferFailed("文件列表不能为空".to_string()));
        }

        let url = format!("{}/1/clouddrive/share/sharepage/save", self.base_url);

        let body = json!({
            "pwd_id": pwd_id,
            "stoken": stoken,
            "fid_list": fid_list,
            "fid_token_list": fid_token_list,
            "target_dir_fid": target_dir_fid
        });

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::TransferFailed(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let resp: ApiResponse<TaskIdData> = response.json().await?;

        match resp.data {
            Some(data) => Ok(data.task_id),
            None => Err(AppError::TransferFailed(resp.message.unwrap_or_default())),
        }
    }

    pub async fn query_transfer_task(&self, task_id: &str) -> Result<TransferTaskStatus, AppError> {
        let url = format!(
            "{}/1/clouddrive/task?pr=1&task_id={}&verify=1",
            self.base_url, task_id
        );

        let response = self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AppError::NetworkError(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let resp: ApiResponse<TransferTaskData> = response.json().await?;

        match resp.data {
            Some(data) => match data.status {
                0 => Ok(TransferTaskStatus::Pending),
                1 => Ok(TransferTaskStatus::Running {
                    progress: data.progress.unwrap_or(0),
                }),
                2 => Ok(TransferTaskStatus::Completed {
                    new_fids: data.new_fids.unwrap_or_default(),
                }),
                3 => Ok(TransferTaskStatus::Failed {
                    reason: data.failed_reason.unwrap_or_else(|| "未知错误".to_string()),
                }),
                _ => Ok(TransferTaskStatus::Running {
                    progress: data.progress.unwrap_or(0),
                }),
            },
            None => Err(AppError::NetworkError(resp.message.unwrap_or_default())),
        }
    }

    pub async fn get_download_link(&self, fid: &str) -> Result<DownloadLink, AppError> {
        let url = format!("{}/1/clouddrive/file/download", self.base_url);

        let body = json!({
            "fid": fid
        });

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if response.status().as_u16() == 404 {
            return Err(AppError::NetworkError("文件不存在".to_string()));
        }

        if !response.status().is_success() {
            return Err(AppError::NetworkError(format!(
                "HTTP {}",
                response.status()
            )));
        }

        #[derive(Deserialize)]
        struct DownloadResponse {
            data: Vec<DownloadLinkData>,
        }

        let resp: DownloadResponse = response.json().await?;

        match resp.data.into_iter().next() {
            Some(link) => Ok(DownloadLink {
                fid: link.fid,
                name: link.name,
                url: link.download_url,
                size: link.size.unwrap_or(0),
                md5: link.md5,
                expires_in: link.expires_in.unwrap_or(0) as u32,
            }),
            None => Err(AppError::NetworkError("未返回下载链接".to_string())),
        }
    }

    pub async fn verify_credential(&self) -> Result<bool, AppError> {
        let url = format!(
            "{}/1/clouddrive/file/sort?dir_fid=0&order_by=2&order_sort=1&page=1&size=1",
            self.base_url
        );

        let mut request = self.client.get(&url);

        if !self.cookie.is_empty() {
            request = request.header("Cookie", &self.cookie);
        }

        let response = request.send().await?;

        if response.status().as_u16() == 401 {
            return Ok(false);
        }

        if !response.status().is_success() {
            return Err(AppError::NetworkError(format!(
                "HTTP {}",
                response.status()
            )));
        }

        Ok(true)
    }

    pub async fn get_user_directories(&self, parent_fid: &str) -> Result<Vec<FileItem>, AppError> {
        let url = format!(
            "{}/1/clouddrive/file/sort?dir_fid={}&order_by=2&order_sort=1&page=1&size=100",
            self.base_url, parent_fid
        );

        let mut request = self.client.get(&url);

        if !self.cookie.is_empty() {
            request = request.header("Cookie", &self.cookie);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(AppError::NetworkError(format!(
                "HTTP {}",
                response.status()
            )));
        }

        #[derive(Deserialize)]
        struct FilesResponse {
            data: FilesData,
        }

        #[derive(Deserialize)]
        struct FilesData {
            list: Vec<FileItem>,
        }

        let resp: FilesResponse = response.json().await?;
        Ok(resp.data.list)
    }
}
