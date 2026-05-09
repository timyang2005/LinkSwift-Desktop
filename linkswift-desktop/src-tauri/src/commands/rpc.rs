//! RPC下载命令模块
//!
//! 提供与RPC下载服务（如Aria2）交互的Tauri命令接口

use crate::services::rpc_client::{RpcClient, RpcTaskStatus};
use crate::services::config_service::ConfigService;
use crate::models::config::RpcServer;
use std::path::PathBuf;

fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("linkswift")
}

fn get_rpc_server_by_id(server_id: &str) -> Option<RpcServer> {
    let config_path = get_config_path();
    let config = ConfigService::load(&config_path).ok()?;
    config.rpc_servers.into_iter().find(|s| s.id == server_id)
}

pub async fn add_download_task(
    rpc_server_id: String,
    urls: Vec<String>,
    filename: String,
    dir: Option<String>,
) -> Result<Vec<String>, String> {
    let server = get_rpc_server_by_id(&rpc_server_id)
        .ok_or_else(|| "未找到指定的RPC服务器".to_string())?;
    
    let client = RpcClient::new(&server.url, server.token.as_deref());
    
    let url_refs: Vec<&str> = urls.iter().map(|s| s.as_str()).collect();
    
    let gid = client.add_uri(url_refs, &filename, dir.as_deref())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(vec![gid])
}

pub async fn test_rpc_connection(url: String, token: Option<String>) -> Result<bool, String> {
    let client = RpcClient::new(&url, token.as_deref());
    
    client.test_connection()
        .await
        .map_err(|e| e.to_string())
}

pub async fn query_rpc_task_status(_rpc_server_id: String, task_id: String) -> Result<RpcTaskStatus, String> {
    let config_path = get_config_path();
    let config = ConfigService::load(&config_path)
        .map_err(|e| e.to_string())?;
    
    let default_idx = config.default_rpc_index;
    let server = config.rpc_servers.get(default_idx)
        .ok_or_else(|| "未找到默认RPC服务器".to_string())?;
    
    let client = RpcClient::new(&server.url, server.token.as_deref());
    
    client.query_task_status(&task_id)
        .await
        .map_err(|e| e.to_string())
}
