//! Config 配置数据模型
//!
//! 定义应用程序配置相关的数据结构

use serde::{Deserialize, Serialize};

/// 应用程序配置
///
/// 包含应用程序的所有配置项
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AppConfig {
    /// 用户认证凭据配置
    pub credential: CredentialConfig,
    /// RPC下载服务器列表
    pub rpc_servers: Vec<RpcServer>,
    /// 默认RPC服务器索引
    pub default_rpc_index: usize,
    /// 应用程序主题设置
    pub theme: Theme,
    /// 代理服务器配置
    pub proxy: Option<ProxyConfig>,
    /// 是否为首次运行
    pub is_first_run: bool,
    /// 下载失败重试次数
    pub retry_count: u32,
}

/// 用户认证凭据配置
///
/// 存储用户的登录凭据信息
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CredentialConfig {
    /// 加密后的Cookie字符串
    pub encrypted_cookie: String,
    /// 最后验证时间戳（Unix时间戳）
    pub last_verified: i64,
    /// 凭据是否有效
    pub is_valid: bool,
    /// 过期前提醒天数
    pub remind_before_expire_days: u32,
}

/// RPC下载服务器配置
///
/// 定义一个RPC下载服务器的信息
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RpcServer {
    /// 服务器唯一标识符
    pub id: String,
    /// 服务器显示名称
    pub name: String,
    /// RPC服务器URL地址
    pub url: String,
    /// RPC认证令牌（可选）
    pub token: Option<String>,
    /// 下载器类型
    pub downloader_type: DownloaderType,
    /// 默认下载目录（可选）
    pub download_dir: Option<String>,
    /// 是否为默认服务器
    pub is_default: bool,
}

/// 下载器类型枚举
///
/// 支持的RPC下载器类型
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum DownloaderType {
    /// Aria2下载器
    Aria2,
    /// BitComet下载器
    BitComet,
    /// ABDownloadManager下载器
    ABDownloadManager,
    /// 自定义下载器
    Custom,
}

/// 代理服务器配置
///
/// HTTP/HTTPS代理设置
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProxyConfig {
    /// 代理服务器URL
    pub url: String,
    /// 代理用户名（可选）
    pub username: Option<String>,
    /// 代理密码（可选）
    pub password: Option<String>,
}

/// 应用程序主题枚举
///
/// 可用的界面主题选项
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Theme {
    /// 浅色主题
    Light,
    /// 深色主题
    Dark,
    /// 跟随系统设置
    System,
}

/// AppConfig的默认实现
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
