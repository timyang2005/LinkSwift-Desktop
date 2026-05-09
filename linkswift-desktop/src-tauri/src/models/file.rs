use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileItem {
    pub fid: String,
    pub name: String,
    pub is_folder: bool,
    pub size: u64,
    pub pdir_fid: String,
    pub mime_type: Option<String>,
    pub file_icon: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub share_fid_token: Option<String>,
    #[serde(default)]
    pub selected: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PaginatedFiles {
    pub items: Vec<FileItem>,
    pub total_count: u64,
    pub page: u32,
    pub page_size: u32,
    pub has_more: bool,
}
