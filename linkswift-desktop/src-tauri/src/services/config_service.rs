use crate::models::config::AppConfig;
use crate::error::AppError;

pub struct ConfigService;

impl ConfigService {
    pub fn save(path: &std::path::Path, config: &AppConfig) -> Result<(), AppError> {
        todo!()
    }

    pub fn load(path: &std::path::Path) -> Result<AppConfig, AppError> {
        todo!()
    }
}
