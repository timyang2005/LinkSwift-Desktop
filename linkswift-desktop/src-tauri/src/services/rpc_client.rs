//! RPC Client Service RPC下载客户端服务
//!
//! 提供与RPC下载服务器（如Aria2）交互的功能封装

use crate::error::AppError;
use crate::models::config::DownloaderType;
use serde_json::json;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RpcTaskStatus {
    Active,
    Waiting,
    Paused,
    Error { message: String },
    Complete,
    Removed,
}

pub struct RpcClient {
    url: String,
    token: Option<String>,
    downloader_type: DownloaderType,
    client: reqwest::Client,
}

impl RpcClient {
    pub fn new(url: &str, token: Option<&str>) -> Self {
        Self {
            url: url.to_string(),
            token: token.map(|s| s.to_string()),
            downloader_type: DownloaderType::Aria2,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_downloader_type(mut self, dt: DownloaderType) -> Self {
        self.downloader_type = dt;
        self
    }

    async fn jsonrpc_call(
        &self,
        method: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, AppError> {
        let method_with_token = match &self.token {
            Some(_token) => format!("aria2.{}", method.trim_start_matches("aria2.")),
            None => method.to_string(),
        };

        let params_with_token: Vec<serde_json::Value> = match &self.token {
            Some(token) => {
                let mut p = vec![serde_json::Value::String(token.clone())];
                p.extend(params);
                p
            }
            None => params,
        };

        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method_with_token,
            "params": params_with_token
        });

        let response = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::RpcConnectionFailed(e.to_string()))?;

        let status = response.status();
        if !status.is_success() && status.as_u16() != 200 {
            return Err(AppError::RpcConnectionFailed(format!("HTTP {}", status)));
        }

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        if let Some(error) = body.get("error") {
            let message = error
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error")
                .to_string();
            return Err(AppError::RpcConnectionFailed(message));
        }

        body.get("result")
            .cloned()
            .ok_or_else(|| AppError::RpcConnectionFailed("No result in response".to_string()))
    }

    pub async fn add_uri(
        &self,
        urls: Vec<&str>,
        _filename: &str,
        dir: Option<&str>,
    ) -> Result<String, AppError> {
        let mut options = serde_json::Map::new();

        if let Some(directory) = dir {
            options.insert(
                "dir".to_string(),
                serde_json::Value::String(directory.to_string()),
            );
        }

        let params = vec![
            serde_json::Value::Array(
                urls.into_iter()
                    .map(|u| serde_json::Value::String(u.to_string()))
                    .collect(),
            ),
            serde_json::Value::Object(options),
        ];

        let result = self.jsonrpc_call("aria2.addUri", params).await?;

        result
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| AppError::RpcConnectionFailed("Expected string result".to_string()))
    }

    pub async fn test_connection(&self) -> Result<bool, AppError> {
        match self.downloader_type {
            DownloaderType::Aria2 => match self.jsonrpc_call("aria2.getVersion", vec![]).await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            },
            DownloaderType::BitComet => {
                match self
                    .client
                    .post(&self.url)
                    .header("Content-Type", "application/json")
                    .json(&json!({"method": "server.ver", "params": [], "id": 1}))
                    .send()
                    .await
                {
                    Ok(resp) => Ok(resp.status().is_success()),
                    Err(_) => Ok(false),
                }
            }
            _ => match self.jsonrpc_call("aria2.getVersion", vec![]).await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            },
        }
    }

    pub async fn query_task_status(&self, task_id: &str) -> Result<RpcTaskStatus, AppError> {
        let params = vec![serde_json::Value::String(task_id.to_string())];

        match self.jsonrpc_call("aria2.tellStatus", params).await {
            Ok(result) => {
                let status = result
                    .get("status")
                    .and_then(|s| s.as_str())
                    .unwrap_or("active");

                match status {
                    "active" => Ok(RpcTaskStatus::Active),
                    "waiting" => Ok(RpcTaskStatus::Waiting),
                    "paused" => Ok(RpcTaskStatus::Paused),
                    "complete" => Ok(RpcTaskStatus::Complete),
                    "removed" => Ok(RpcTaskStatus::Removed),
                    "error" => {
                        let message = result
                            .get("errorMessage")
                            .and_then(|m| m.as_str())
                            .unwrap_or("Unknown error")
                            .to_string();
                        Ok(RpcTaskStatus::Error { message })
                    }
                    _ => Ok(RpcTaskStatus::Active),
                }
            }
            Err(e) => Err(e),
        }
    }
}
