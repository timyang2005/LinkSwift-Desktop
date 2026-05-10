//! LinkSwift Desktop - 夸克网盘链接下载工具核心库
//!
//! 本库是 LinkSwift Desktop 应用的核心模块，提供了与夸克网盘交互的完整功能：
//! - 分享链接解析与文件获取
//! - 文件转存服务
//! - RPC下载任务管理
//! - 配置管理与加密服务

pub mod commands;
pub mod error;
pub mod models;
pub mod services;

/// Tauri应用程序入口函数
///
/// 初始化Tauri构建器并配置应用程序：
/// - 注册所有前端可调用的命令
/// - 在调试模式下启用日志插件
/// - 设置应用程序的生命周期管理
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::quark::parse_share_link,
            commands::quark::submit_share_password,
            commands::quark::get_share_files,
            commands::quark::transfer_files,
            commands::quark::query_transfer_task,
            commands::quark::get_download_link,
            commands::quark::verify_credential,
            commands::quark::get_user_directories,
            commands::rpc::add_download_task,
            commands::rpc::test_rpc_connection,
            commands::rpc::query_rpc_task_status,
            commands::config::get_config,
            commands::config::save_config,
            commands::config::add_rpc_server,
            commands::config::update_rpc_server,
            commands::config::delete_rpc_server,
            commands::config::set_default_rpc_server,
            commands::auth::open_login_window,
            commands::auth::verify_credential_status,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
