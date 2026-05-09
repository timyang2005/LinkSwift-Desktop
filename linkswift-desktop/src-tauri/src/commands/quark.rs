//! Quark 夸克网盘命令模块
//!
//! 提供与夸克网盘交互的Tauri命令接口

use crate::error::AppError;
use crate::models::share::ShareInfo;
use crate::models::file::PaginatedFiles;
use crate::models::task::DownloadLink;
use crate::services::quark_api::{QuarkApi, TransferTaskStatus};
use crate::services::config_service::ConfigService;
use std::path::PathBuf;

fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("linkswift")
}

pub async fn parse_share_link(share_url: String) -> Result<ShareInfo, String> {
    let pwd_id = QuarkApi::parse_share_url(&share_url)
        .map_err(|e| e.to_string())?;
    
    Ok(ShareInfo {
        pwd_id,
        stoken: String::new(),
        title: String::new(),
        has_password: false,
        files: vec![],
    })
}

pub async fn submit_share_password(pwd_id: String, _stoken: String, password: String) -> Result<ShareInfo, String> {
    let api = QuarkApi::new("https://drive-pc.quark.cn");
    
    let passcode = if password.is_empty() { None } else { Some(password.as_str()) };
    
    let _stoken = api.get_share_token(&pwd_id, passcode)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(ShareInfo {
        pwd_id,
        stoken: _stoken,
        title: String::new(),
        has_password: false,
        files: vec![],
    })
}

pub async fn get_share_files(pwd_id: String, stoken: String, dir_fid: String, page: u32, size: u32) -> Result<PaginatedFiles, String> {
    let api = QuarkApi::new("https://drive-pc.quark.cn");
    
    api.get_share_files(&pwd_id, &stoken, &dir_fid, page, size)
        .await
        .map_err(|e| e.to_string())
}

pub async fn transfer_files(
    pwd_id: String,
    stoken: String,
    fid_list: Vec<String>,
    fid_token_list: Vec<String>,
    target_dir_fid: String,
) -> Result<String, String> {
    if fid_list.is_empty() {
        return Err("文件列表不能为空".to_string());
    }
    
    let api = QuarkApi::new("https://drive-pc.quark.cn");
    
    api.transfer_files(&pwd_id, &stoken, &fid_list, &fid_token_list, &target_dir_fid)
        .await
        .map_err(|e| e.to_string())
}

pub async fn query_transfer_task(task_id: String) -> Result<TransferTaskStatus, String> {
    let api = QuarkApi::new("https://drive-pc.quark.cn");
    
    api.query_transfer_task(&task_id)
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_download_link(fid: String) -> Result<DownloadLink, String> {
    let api = QuarkApi::new("https://drive-pc.quark.cn");
    
    api.get_download_link(&fid)
        .await
        .map_err(|e| e.to_string())
}

pub async fn verify_credential() -> Result<bool, String> {
    let config_path = get_config_path();
    let config = ConfigService::load(&config_path)
        .map_err(|e| e.to_string())?;
    
    if config.credential.encrypted_cookie.is_empty() {
        return Ok(false);
    }
    
    let api = QuarkApi::new("https://drive-pc.quark.cn")
        .with_cookie(&config.credential.encrypted_cookie);
    
    api.verify_credential()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_user_directories(parent_fid: String) -> Result<Vec<crate::models::file::FileItem>, String> {
    let config_path = get_config_path();
    let config = ConfigService::load(&config_path)
        .map_err(|e| e.to_string())?;
    
    let api = QuarkApi::new("https://drive-pc.quark.cn")
        .with_cookie(&config.credential.encrypted_cookie);
    
    api.get_user_directories(&parent_fid)
        .await
        .map_err(|e| e.to_string())
}
