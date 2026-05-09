//! Commands 命令模块
//!
//! 定义Tauri命令接口，供前端调用
//!
//! 模块结构：
//! - quark: 夸克网盘相关命令
//! - rpc: RPC下载相关命令
//! - config: 配置管理命令
//! - auth: 认证相关命令

pub mod quark;
pub mod rpc;
pub mod config;
pub mod auth;
