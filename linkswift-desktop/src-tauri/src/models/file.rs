//! File 文件数据模型
//!
//! 定义文件和目录相关的数据结构

use serde::{Deserialize, Serialize};

/// 文件或目录项
///
/// 表示网盘中的一个文件或文件夹
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileItem {
    /// 文件唯一标识符
    pub fid: String,
    /// 文件或目录名称
    pub name: String,
    /// 是否为文件夹
    pub is_folder: bool,
    /// 文件大小（字节）
    pub size: u64,
    /// 父目录ID
    pub pdir_fid: String,
    /// MIME类型（文件专用）
    pub mime_type: Option<String>,
    /// 文件图标标识
    pub file_icon: Option<String>,
    /// 创建时间戳（Unix时间戳）
    pub created_at: Option<i64>,
    /// 更新时间戳（Unix时间戳）
    pub updated_at: Option<i64>,
    /// 分享文件令牌
    pub share_fid_token: Option<String>,
    /// 是否选中（用于前端UI）
    #[serde(default)]
    pub selected: bool,
}

/// 分页文件列表
///
/// 包含文件列表和分页信息
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PaginatedFiles {
    /// 文件项列表
    pub items: Vec<FileItem>,
    /// 总文件数量
    pub total_count: u64,
    /// 当前页码
    pub page: u32,
    /// 每页大小
    pub page_size: u32,
    /// 是否还有更多数据
    pub has_more: bool,
}
