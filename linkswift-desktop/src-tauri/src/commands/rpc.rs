use crate::services::rpc_client::RpcTaskStatus;

pub async fn add_download_task(
    rpc_server_id: String,
    urls: Vec<String>,
    filename: String,
    dir: Option<String>,
) -> Result<Vec<String>, String> {
    todo!()
}

pub async fn test_rpc_connection(url: String, token: Option<String>) -> Result<bool, String> {
    todo!()
}

pub async fn query_rpc_task_status(rpc_server_id: String, task_id: String) -> Result<RpcTaskStatus, String> {
    todo!()
}
