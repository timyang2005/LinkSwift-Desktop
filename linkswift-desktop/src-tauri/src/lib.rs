//! LinkSwift Desktop - 夸克网盘链接下载工具核心库
//!
//! 本库是 LinkSwift Desktop 应用的核心模块，提供了与夸克网盘交互的完整功能：
//! - 分享链接解析与文件获取
//! - 文件转存服务
//! - RPC下载任务管理
//! - 配置管理与加密服务

pub mod models;
pub mod commands;
pub mod services;
pub mod error;

/// Tauri应用程序入口函数
///
/// 初始化Tauri构建器并配置应用程序：
/// - 在调试模式下启用日志插件
/// - 设置应用程序的生命周期管理
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
