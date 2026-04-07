//! # Simpfun SDK
//!
//! Simpfun API 的 Rust SDK，提供登录、用户信息、实例管理、开发者镜像接口、WebSocket 控制等功能。
//!
//! ## 快速开始
//!
//! ```rust,no_run
//! use simpfun_sdk::{SimpfunClient, SdkError};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), SdkError> {
//!     let client = SimpfunClient::new()?;
//!     client.user().login_and_set_token("username", "password").await?;
//!
//!     let info = client.user().auth_info().await?;
//!     println!("用户: {:?}", info.info.username);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 功能特性
//!
//! - HTTP API 完整封装（登录、实例管理、文件操作等）
//! - 开发者专属 API（开发者镜像列表、详情、版本、反馈）
//! - WebSocket 实时事件流（控制台输出、状态变更等）
//! - 事件轮询系统（公告、积分、实例列表变更通知）
//! - 自动重连与 Token 刷新

mod error;
mod models;
mod client;
mod events;
mod ws;

pub use crate::client::SimpfunClient;
pub use crate::error::SdkError;
pub use crate::models::*;
pub use crate::events::{
    EventManager,
    Event,
    EventIntervals,
    EventStop,
    EventControl,
    EventCmd,
    Topic,
    PollingConfig,
};
pub use crate::ws::{WsEvent, WsControl, WsStop, connect_ins_ws};
