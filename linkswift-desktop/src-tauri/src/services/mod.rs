//! Services 服务模块
//!
//! 提供应用程序的核心业务逻辑服务
//!
//! 模块结构：
//! - quark_api: 夸克网盘API交互服务
//! - rpc_client: RPC下载客户端服务
//! - crypto: 加密解密服务
//! - config_service: 配置文件服务
//! - task_queue: 任务队列管理服务

pub mod quark_api;
pub mod rpc_client;
pub mod crypto;
pub mod config_service;
pub mod task_queue;
