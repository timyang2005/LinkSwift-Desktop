//! Config 配置管理命令模块
//!
//! 提供应用程序配置相关的Tauri命令接口

use crate::models::config::{AppConfig, RpcServer};
use crate::services::config_service::ConfigService;
use std::path::PathBuf;

fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("linkswift")
}

pub async fn get_config() -> Result<AppConfig, String> {
    let config_path = get_config_path();
    
    ConfigService::load(&config_path)
        .map_err(|e| e.to_string())
}

pub async fn save_config(config: AppConfig) -> Result<(), String> {
    let config_path = get_config_path();
    
    ConfigService::save(&config_path, &config)
        .map_err(|e| e.to_string())
}

pub async fn add_rpc_server(server: RpcServer) -> Result<(), String> {
    let config_path = get_config_path();
    let mut config = ConfigService::load(&config_path)
        .map_err(|e| e.to_string())?;
    
    config.rpc_servers.push(server);
    
    ConfigService::save(&config_path, &config)
        .map_err(|e| e.to_string())
}

pub async fn update_rpc_server(server: RpcServer) -> Result<(), String> {
    let config_path = get_config_path();
    let mut config = ConfigService::load(&config_path)
        .map_err(|e| e.to_string())?;
    
    if let Some(existing) = config.rpc_servers.iter_mut().find(|s| s.id == server.id) {
        *existing = server;
    }
    
    ConfigService::save(&config_path, &config)
        .map_err(|e| e.to_string())
}

pub async fn delete_rpc_server(server_id: String) -> Result<(), String> {
    let config_path = get_config_path();
    let mut config = ConfigService::load(&config_path)
        .map_err(|e| e.to_string())?;
    
    config.rpc_servers.retain(|s| s.id != server_id);
    
    ConfigService::save(&config_path, &config)
        .map_err(|e| e.to_string())
}

pub async fn set_default_rpc_server(server_id: String) -> Result<(), String> {
    let config_path = get_config_path();
    let mut config = ConfigService::load(&config_path)
        .map_err(|e| e.to_string())?;
    
    let mut found = false;
    for (idx, server) in config.rpc_servers.iter().enumerate() {
        if server.id == server_id {
            for s in &mut config.rpc_servers {
                s.is_default = false;
            }
            config.rpc_servers[idx].is_default = true;
            config.default_rpc_index = idx;
            found = true;
            break;
        }
    }
    
    if !found {
        return Err(format!("RPC server with id '{}' not found", server_id));
    }
    
    ConfigService::save(&config_path, &config)
        .map_err(|e| e.to_string())
}
