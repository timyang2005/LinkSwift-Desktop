use crate::models::config::AppConfig;
use crate::models::config::RpcServer;

pub async fn get_config() -> Result<AppConfig, String> {
    todo!()
}

pub async fn save_config(config: AppConfig) -> Result<(), String> {
    todo!()
}

pub async fn add_rpc_server(server: RpcServer) -> Result<(), String> {
    todo!()
}

pub async fn update_rpc_server(server: RpcServer) -> Result<(), String> {
    todo!()
}

pub async fn delete_rpc_server(server_id: String) -> Result<(), String> {
    todo!()
}

pub async fn set_default_rpc_server(server_id: String) -> Result<(), String> {
    todo!()
}
