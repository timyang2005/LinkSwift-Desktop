use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AppConfig {
    pub credential: CredentialConfig,
    pub rpc_servers: Vec<RpcServer>,
    pub default_rpc_index: usize,
    pub theme: Theme,
    pub proxy: Option<ProxyConfig>,
    pub is_first_run: bool,
    pub retry_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CredentialConfig {
    pub encrypted_cookie: String,
    pub last_verified: i64,
    pub is_valid: bool,
    pub remind_before_expire_days: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RpcServer {
    pub id: String,
    pub name: String,
    pub url: String,
    pub token: Option<String>,
    pub downloader_type: DownloaderType,
    pub download_dir: Option<String>,
    pub is_default: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DownloaderType {
    Aria2,
    BitComet,
    ABDownloadManager,
    Custom,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProxyConfig {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            credential: CredentialConfig {
                encrypted_cookie: String::new(),
                last_verified: 0,
                is_valid: false,
                remind_before_expire_days: 7,
            },
            rpc_servers: Vec::new(),
            default_rpc_index: 0,
            theme: Theme::System,
            proxy: None,
            is_first_run: true,
            retry_count: 3,
        }
    }
}
