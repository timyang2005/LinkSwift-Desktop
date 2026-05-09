//! Crypto Service 加密解密服务
//!
//! 提供敏感数据的加密和解密功能

use crate::error::AppError;

/// 加密服务
///
/// 用于安全存储敏感信息（如Cookie）
/// 使用Base64编码作为跨平台加密方案
pub struct CryptoService;

impl CryptoService {
    /// 加密数据
    ///
    /// 使用Base64编码对明文数据进行加密
    ///
    /// # 参数
    /// * `data` - 要加密的明文字符串
    ///
    /// # 返回值
    /// * `Ok(String)` - 加密后的密文字符串
    /// * `Err(AppError)` - 加密失败
    pub fn encrypt(data: &str) -> Result<String, AppError> {
        Ok(base64_encode(data.as_bytes()))
    }

    /// 解密数据
    ///
    /// 对加密后的密文进行解密
    ///
    /// # 参数
    /// * `encrypted` - 加密后的密文字符串
    ///
    /// # 返回值
    /// * `Ok(String)` - 解密后的明文字符串
    /// * `Err(AppError)` - 解密失败（无效数据）
    pub fn decrypt(encrypted: &str) -> Result<String, AppError> {
        let bytes = base64_decode(encrypted)
            .map_err(|_| AppError::CryptoError("无效的加密数据".to_string()))?;
        String::from_utf8(bytes)
            .map_err(|_| AppError::CryptoError("解密后数据不是有效的UTF-8字符串".to_string()))
    }
}

fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = chunk.get(1).copied().unwrap_or(0) as usize;
        let b2 = chunk.get(2).copied().unwrap_or(0) as usize;
        
        result.push(ALPHABET[(b0 >> 2)] as char);
        result.push(ALPHABET[((b0 & 0x03) << 4) | (b1 >> 4)] as char);
        
        if chunk.len() > 1 {
            result.push(ALPHABET[((b1 & 0x0f) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(ALPHABET[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}

fn base64_decode(input: &str) -> Result<Vec<u8>, ()> {
    const DECODE_TABLE: [i8; 128] = [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 62, -1, -1, -1, 63,
        52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1, -1,
        -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
        15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1,
        -1, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, -1, -1, -1, -1, -1,
    ];
    
    let input = input.trim_end_matches('=');
    if input.is_empty() {
        return Ok(Vec::new());
    }
    
    let chars: Vec<u8> = input.bytes().collect();
    for &c in &chars {
        if c >= 128 || (c as i8) < 0 || DECODE_TABLE[c as usize] < 0 {
            return Err(());
        }
    }
    
    let mut result = Vec::with_capacity(chars.len() * 3 / 4);
    
    for chunk in chars.chunks(4) {
        let mut buf = [0u8; 4];
        for (i, &c) in chunk.iter().enumerate() {
            buf[i] = DECODE_TABLE[c as usize] as u8;
        }
        
        result.push((buf[0] << 2) | (buf[1] >> 4));
        if chunk.len() > 2 {
            result.push((buf[1] << 4) | (buf[2] >> 2));
        }
        if chunk.len() > 3 {
            result.push((buf[2] << 6) | buf[3]);
        }
    }
    
    Ok(result)
}
