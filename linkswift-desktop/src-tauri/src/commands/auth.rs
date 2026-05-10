//! Auth 认证命令模块
//!
//! 提供用户认证相关的Tauri命令接口

use crate::services::config_service::ConfigService;
use std::path::PathBuf;

fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("linkswift")
}

pub async fn open_login_window() -> Result<String, String> {
    Ok("登录窗口已打开".to_string())
}

pub async fn verify_credential_status() -> Result<bool, String> {
    let config_path = get_config_path();
    let config = ConfigService::load(&config_path).map_err(|e| e.to_string())?;

    if config.credential.encrypted_cookie.is_empty() {
        return Ok(false);
    }

    if !config.credential.is_valid {
        return Ok(false);
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let expire_threshold = config.credential.remind_before_expire_days as i64 * 86400;
    if config.credential.last_verified > 0
        && (now - config.credential.last_verified) > expire_threshold
    {
        return Ok(false);
    }

    Ok(true)
}
