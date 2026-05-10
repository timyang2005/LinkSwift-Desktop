//! LinkSwift Desktop 主程序入口
//!
//! 本文件是应用程序的入口点，负责：
//! - 阻止Windows release模式下额外的控制台窗口
//! - 调用核心库启动Tauri应用程序

// 在release模式下隐藏Windows控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// 应用程序主函数
///
/// 通过调用 app_lib::run() 启动Tauri应用程序
fn main() {
    app_lib::run();
}
