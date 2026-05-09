//! Task 任务数据模型
//!
//! 定义下载任务和任务状态相关的数据结构

use serde::{Deserialize, Serialize};
use crate::models::file::FileItem;

/// 下载任务
///
/// 表示一个完整的下载任务
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DownloadTask {
    /// 任务唯一标识符
    pub id: String,
    /// 分享链接URL
    pub share_url: String,
    /// 要下载的文件列表
    pub files: Vec<FileItem>,
    /// 当前任务状态
    pub status: TaskStatus,
    /// 目标下载目录
    pub target_dir: String,
    /// 使用的RPC服务器ID
    pub rpc_server_id: String,
    /// 任务创建时间戳
    pub created_at: i64,
    /// 错误信息（如果有）
    pub error_message: Option<String>,
    /// 已重试次数
    pub retry_count: u32,
}

/// 任务状态枚举
///
/// 下载任务的可能状态
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskStatus {
    /// 等待处理
    Pending,
    /// 正在解析链接
    Parsing,
    /// 正在转存中（包含进度百分比）
    Transferring { progress: u32 },
    /// 正在获取下载链接
    FetchingLink,
    /// 正在推送到RPC
    Pushing,
    /// 任务完成
    Completed,
    /// 任务失败（包含失败原因）
    Failed { reason: String },
    /// 任务已取消
    Cancelled,
}

/// 下载链接信息
///
/// 包含文件的直接下载链接
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DownloadLink {
    /// 文件ID
    pub fid: String,
    /// 文件名
    pub name: String,
    /// 下载URL
    pub url: String,
    /// 文件大小
    pub size: u64,
    /// MD5校验值（可选）
    pub md5: Option<String>,
    /// 链接有效期（秒）
    pub expires_in: u32,
}
