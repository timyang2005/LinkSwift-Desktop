//! Config Service 配置文件服务
//!
//! 负责应用程序配置的读取和保存

use crate::error::AppError;
use crate::models::config::AppConfig;

/// 配置服务
///
/// 提供配置的持久化存储功能
pub struct ConfigService;

impl ConfigService {
    /// 保存配置到文件
    ///
    /// 将AppConfig对象序列化为JSON并写入指定路径
    ///
    /// # 参数
    /// * `path` - 配置文件目录路径
    /// * `config` - 要保存的配置对象
    ///
    /// # 返回值
    /// * `Ok(())` - 保存成功
    /// * `Err(AppError)` - 保存失败
    pub fn save(path: &std::path::Path, config: &AppConfig) -> Result<(), AppError> {
        std::fs::create_dir_all(path)?;
        let file_path = path.join("config.json");
        let json = serde_json::to_string_pretty(config)
            .map_err(|e| AppError::ConfigError(format!("序列化配置失败: {}", e)))?;
        std::fs::write(file_path, json)?;
        Ok(())
    }

    /// 从文件加载配置
    ///
    /// 从指定路径读取配置文件并反序列化为AppConfig对象
    ///
    /// # 参数
    /// * `path` - 配置文件目录路径
    ///
    /// # 返回值
    /// * `Ok(AppConfig)` - 加载成功返回配置对象
    /// * `Err(AppError)` - 加载失败
    pub fn load(path: &std::path::Path) -> Result<AppConfig, AppError> {
        let file_path = path.join("config.json");
        if !file_path.exists() {
            return Ok(AppConfig::default());
        }
        let json = std::fs::read_to_string(file_path)?;
        let config: AppConfig = serde_json::from_str(&json)
            .map_err(|e| AppError::ConfigError(format!("解析配置文件失败: {}", e)))?;
        Ok(config)
    }
}
