use crate::error::AppError;

pub struct CryptoService;

impl CryptoService {
    pub fn encrypt(data: &str) -> Result<String, AppError> {
        todo!()
    }

    pub fn decrypt(encrypted: &str) -> Result<String, AppError> {
        todo!()
    }
}
