use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Cookie无效或已过期 (E001)")]
    InvalidCookie,

    #[error("链接解析失败 (E002): {0}")]
    ParseError(String),

    #[error("分享已失效 (E003): {0}")]
    ShareExpired(String),

    #[error("转存失败 (E004): {0}")]
    TransferFailed(String),

    #[error("RPC连接失败 (E005): {0}")]
    RpcConnectionFailed(String),

    #[error("网络错误 (E006): {0}")]
    NetworkError(String),

    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("加密错误: {0}")]
    CryptoError(String),

    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("序列化错误: {0}")]
    SerializeError(#[from] serde_json::Error),

    #[error("HTTP请求错误: {0}")]
    RequestError(#[from] reqwest::Error),
}
