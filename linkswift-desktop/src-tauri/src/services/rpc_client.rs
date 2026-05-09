//! RPC Client Service RPC下载客户端服务
//!
//! 提供与RPC下载服务器（如Aria2）交互的功能封装

use crate::error::AppError;
use crate::models::config::{DownloaderType, RpcServer};

/// RPC下载客户端
///
/// 封装与各种RPC下载器的交互
pub struct RpcClient {
    /// RPC服务器URL
    url: String,
    /// RPC认证令牌
    token: Option<String>,
    /// 下载器类型
    downloader_type: DownloaderType,
}

impl RpcClient {
    /// 创建新的RPC客户端实例
    ///
    /// # 参数
    /// * `url` - RPC服务器URL
    /// * `token` - 认证令牌（可选）
    ///
    /// # 返回值
    /// 新的RpcClient实例
    pub fn new(url: &str, token: Option<&str>) -> Self {
        Self {
            url: url.to_string(),
            token: token.map(|s| s.to_string()),
            downloader_type: DownloaderType::Aria2,
        }
    }

    /// 设置下载器类型
    ///
    /// 返回配置了下载器类型的新实例（Builder模式）
    ///
    /// # 参数
    /// * `dt` - 下载器类型
    ///
    /// # 返回值
    /// 配置了下载器类型的新实例
    pub fn with_downloader_type(mut self, dt: DownloaderType) -> Self {
        self.downloader_type = dt;
        self
    }

    /// 添加下载任务（URI方式）
    ///
    /// 通过RPC接口添加新的下载任务
    ///
    /// # 参数
    /// * `urls` - 下载URL列表
    /// * `filename` - 文件名
    /// * `dir` - 下载目录（可选）
    ///
    /// # 返回值
    /// * `Ok(String)` - 添加的任务ID
    /// * `Err(AppError)` - 添加失败
    pub async fn add_uri(&self, urls: Vec<&str>, filename: &str, dir: Option<&str>) -> Result<String, AppError> {
        todo!()
    }

    /// 测试RPC连接
    ///
    /// 验证RPC服务器是否可达
    ///
    /// # 返回值
    /// * `Ok(bool)` - true表示连接正常
    /// * `Err(AppError)` - 连接失败
    pub async fn test_connection(&self) -> Result<bool, AppError> {
        todo!()
    }

    /// 查询任务状态
    ///
    /// 获取指定任务的状态信息
    ///
    /// # 参数
    /// * `task_id` - 任务ID
    ///
    /// # 返回值
    /// * `Ok(RpcTaskStatus)` - 任务状态
    /// * `Err(AppError)` - 查询失败
    pub async fn query_task_status(&self, task_id: &str) -> Result<RpcTaskStatus, AppError> {
        todo!()
    }
}

/// RPC任务状态枚举
///
/// 表示RPC下载服务器上的任务状态
#[derive(Debug, Clone, PartialEq)]
pub enum RpcTaskStatus {
    /// 正在下载
    Active,
    /// 等待中
    Waiting,
    /// 已暂停
    Paused,
    /// 下载错误（包含错误信息）
    Error { message: String },
    /// 已完成
    Complete,
    /// 已删除
    Removed,
}
