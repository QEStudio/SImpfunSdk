//! Simpfun API 客户端模块

mod http;
mod auth;
mod instance;
mod file;
mod backup;
mod shop;
mod games;

use std::sync::Arc;
use std::time::Duration;

use arc_swap::ArcSwapOption;
use reqwest::Client;

use crate::error::SdkError;

pub use http::ResourceMeta;

#[derive(Clone)]
/// Simpfun API 客户端
/// 
/// 提供所有 Simpfun API 的封装，包括：
/// - 用户认证（登录、登出）
/// - 实例管理（列表、详情、电源控制）
/// - 文件操作（列表、读取、保存、压缩等）
/// - 商店与订阅管理
/// 
/// # 示例
/// 
/// ```rust,no_run
/// use simpfun_sdk::SimpfunClient;
/// 
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = SimpfunClient::new()?;
/// client.set_token("your-token");
/// # Ok(())
/// # }
/// ```
pub struct SimpfunClient {
    pub(crate) base_url: String,
    pub(crate) http: Client,
    pub(crate) referer: Option<String>,
    pub(crate) token: Arc<ArcSwapOption<String>>,
}

impl SimpfunClient {
    /// 创建默认客户端（基地址为 `https://api.simpfun.cn`），Referer 为 `https://simpfun.cn/`
    pub fn new() -> Result<Self, SdkError> {
        let http = Client::builder()
            .timeout(Duration::from_secs(15))
            .user_agent("simpfun-sdk-rust/0.1")
            .pool_idle_timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(32)
            .tcp_nodelay(true)
            .build()?;

        Ok(Self {
            base_url: "https://api.simpfun.cn".to_string(),
            http,
            referer: Some("https://simpfun.cn/".to_string()),
            token: Arc::new(ArcSwapOption::from(None)),
        })
    }

    /// 自定义基地址
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// 自定义 Referer
    pub fn set_referer(mut self, referer: impl Into<String>) -> Self {
        self.referer = Some(referer.into());
        self
    }

    /// 设置令牌（从登录返回或手动提供）
    pub fn set_token(&self, token: impl Into<String>) {
        self.token.store(Some(Arc::new(token.into())));
    }

    /// 清除当前令牌
    pub fn clear_token(&self) {
        self.token.store(None);
    }

    /// 获取当前令牌（如设置过）
    pub fn token(&self) -> Option<String> {
        self.token.load().as_ref().map(|t| (**t).clone())
    }
}
