// url_validator.rs

use reqwest; // 用于发送HTTP请求
use tokio; // 异步运行时
use std::error::Error;
use std::fmt;
use serde::Deserialize; // 用于JSON反序列化

// 定义一个结构体用于存储URL验证结果
#[derive(Debug, Deserialize)]
struct UrlValidationResponse {
    success: bool,
    message: String,
}

// 定义一个错误类型，用于处理URL验证过程中的错误
#[derive(Debug, Clone)]
struct UrlValidationError(String);

impl fmt::Display for UrlValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for UrlValidationError {}

// 异步函数，用于验证URL的有效性
async fn validate_url(url: &str) -> Result<UrlValidationResponse, UrlValidationError> {
    // 发送HEAD请求到指定的URL
    let response = reqwest::head(url)
        .await
        .map_err(|e| UrlValidationError(format!(