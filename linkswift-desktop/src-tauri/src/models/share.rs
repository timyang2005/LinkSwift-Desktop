//! Share 分享数据模型
//!
//! 定义分享链接相关的数据结构

use serde::{Deserialize, Serialize};
use crate::models::file::FileItem;

/// 分享信息
///
/// 表示一个分享链接的完整信息
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ShareInfo {
    /// 分享ID
    pub pwd_id: String,
    /// 安全令牌
    pub stoken: String,
    /// 分享标题
    pub title: String,
    /// 是否设置密码保护
    pub has_password: bool,
    /// 分享中的文件列表
    pub files: Vec<FileItem>,
}

/// 分享令牌请求
///
/// 请求获取分享访问令牌
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ShareTokenRequest {
    /// 分享ID
    pub pwd_id: String,
    /// 分享密码（如果有）
    pub passcode: Option<String>,
}

/// 分享令牌响应
///
/// 包含获取到的安全令牌
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ShareTokenResponse {
    /// 安全令牌
    pub stoken: String,
}
