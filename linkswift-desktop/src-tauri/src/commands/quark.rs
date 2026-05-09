use crate::error::AppError;
use crate::models::share::ShareInfo;
use crate::models::file::PaginatedFiles;
use crate::models::task::DownloadLink;
use crate::services::quark_api::TransferTaskStatus;

pub async fn parse_share_link(share_url: String) -> Result<ShareInfo, String> {
    todo!()
}

pub async fn submit_share_password(pwd_id: String, stoken: String, password: String) -> Result<ShareInfo, String> {
    todo!()
}

pub async fn get_share_files(pwd_id: String, stoken: String, dir_fid: String, page: u32, size: u32) -> Result<PaginatedFiles, String> {
    todo!()
}

pub async fn transfer_files(
    pwd_id: String,
    stoken: String,
    fid_list: Vec<String>,
    fid_token_list: Vec<String>,
    target_dir_fid: String,
) -> Result<String, String> {
    todo!()
}

pub async fn query_transfer_task(task_id: String) -> Result<TransferTaskStatus, String> {
    todo!()
}

pub async fn get_download_link(fid: String) -> Result<DownloadLink, String> {
    todo!()
}

pub async fn verify_credential() -> Result<bool, String> {
    todo!()
}

pub async fn get_user_directories(parent_fid: String) -> Result<Vec<crate::models::file::FileItem>, String> {
    todo!()
}
