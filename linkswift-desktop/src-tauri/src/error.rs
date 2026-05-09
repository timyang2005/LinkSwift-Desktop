//! 应用程序错误类型定义
//!
//! 定义了应用程序可能遇到的各种错误类型，包括：
//! - 认证相关错误（Cookie失效、分享过期）
//! - 网络和RPC连接错误
//! - 配置和加密错误
//! - IO和序列化错误

use thiserror::Error;

/// 应用程序错误枚举
///
/// 包含所有可能出现的错误类型，每个变体都有对应的错误代码
#[derive(Error, Debug)]
pub enum AppError {
    /// Cookie无效或已过期
    #[error("Cookie无效或已过期 (E001)")]
    InvalidCookie,

    /// 分享链接解析失败
    #[error("链接解析失败 (E002): {0}")]
    ParseError(String),

    /// 分享内容已失效
    #[error("分享已失效 (E003): {0}")]
    ShareExpired(String),

    /// 文件转存操作失败
    #[error("转存失败 (E004): {0}")]
    TransferFailed(String),

    /// RPC服务器连接失败
    #[error("RPC连接失败 (E005): {0}")]
    RpcConnectionFailed(String),

    /// 网络请求错误
    #[error("网络错误 (E006): {0}")]
    NetworkError(String),

    /// 配置读取或保存错误
    #[error("配置错误: {0}")]
    ConfigError(String),

    /// 加密/解密操作错误
    #[error("加密错误: {0}")]
    CryptoError(String),

    /// IO操作错误（文件读写等）
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON序列化/反序列化错误
    #[error("序列化错误: {0}")]
    SerializeError(#[from] serde_json::Error),

    /// HTTP请求错误
    #[error("HTTP请求错误: {0}")]
    RequestError(#[from] reqwest::Error),

    /// 任务队列操作错误
    #[error("任务队列错误: {0}")]
    TaskQueueError(String),
}
