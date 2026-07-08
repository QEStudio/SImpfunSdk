//! Simpfun API 客户端模块

pub mod dev;
mod http;
pub mod user;

use std::sync::Arc;
use std::time::Duration;

use arc_swap::ArcSwapOption;
use reqwest::Client;

use crate::error::SdkError;

pub use dev::DevClient;
pub use http::ResourceMeta;
pub use user::UserClient;

#[derive(Clone)]
pub struct SimpfunClient {
    pub(crate) base_url: String,
    pub(crate) http: Client,
    pub(crate) referer: Option<String>,
    pub(crate) token: Arc<ArcSwapOption<String>>,
}

impl SimpfunClient {
    /// 创建默认客户端
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

    /// 链式设置 token
    pub fn with_token(self, token: impl Into<String>) -> Self {
        self.set_token(token);
        self
    }

    /// 自定义 Referer
    pub fn set_referer(mut self, referer: impl Into<String>) -> Self {
        self.referer = Some(referer.into());
        self
    }

    /// 设置令牌
    pub fn set_token(&self, token: impl Into<String>) {
        self.token.store(Some(Arc::new(token.into())));
    }

    /// 清除令牌
    pub fn clear_token(&self) {
        self.token.store(None);
    }

    /// 获取当前令牌
    pub fn token(&self) -> Option<String> {
        self.token.load().as_ref().map(|t| (**t).clone())
    }

    /// 用户侧 API
    pub fn user(&self) -> UserClient<'_> {
        UserClient::new(self)
    }

    /// 开发者侧 API
    pub fn dev(&self) -> DevClient<'_> {
        DevClient::new(self)
    }
}
