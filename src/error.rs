//! SDK 错误类型定义

use reqwest::StatusCode;
use thiserror::Error;
use tokio_tungstenite::tungstenite::Error as WsError;

/// SDK 统一错误类型
#[derive(Debug, Error)]
pub enum SdkError {
    /// HTTP 请求错误
    #[error("HTTP错误: {0}")]
    Http(Box<reqwest::Error>),

    /// JSON 序列化/反序列化错误
    #[error("序列化/反序列化错误: {0}")]
    Serde(Box<serde_json::Error>),

    /// URL 编码错误
    #[error("UrlEncoded错误: {0}")]
    UrlEncoded(#[from] serde_urlencoded::ser::Error),

    /// 缺少认证令牌
    #[error("缺少令牌，请先登录或设置令牌")]
    MissingToken,

    /// API 业务错误
    #[error("API错误，code={code}, msg={msg}")]
    Api { code: i32, msg: String },

    /// HTTP 状态码错误
    #[error("非预期HTTP状态码: {status}, 响应: {body}")]
    Status { status: StatusCode, body: String },

    /// 认证失败
    #[error("认证失败: Token 无效或已过期")]
    AuthFailed,

    /// WebSocket 错误
    #[error("WebSocket错误: {0}")]
    Ws(Box<WsError>),
}

impl From<reqwest::Error> for SdkError {
    fn from(e: reqwest::Error) -> Self {
        SdkError::Http(Box::new(e))
    }
}

impl From<serde_json::Error> for SdkError {
    fn from(e: serde_json::Error) -> Self {
        SdkError::Serde(Box::new(e))
    }
}

impl From<WsError> for SdkError {
    fn from(e: WsError) -> Self {
        SdkError::Ws(Box::new(e))
    }
}
