//! HTTP 基础设施

use std::time::Duration;

use reqwest::StatusCode;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, REFERER, ETAG};
use tokio::time::sleep;

use crate::error::SdkError;
use super::SimpfunClient;

#[derive(Debug, Clone)]
pub struct ResourceMeta {
    pub len: Option<u64>,
    pub etag: Option<String>,
}

impl SimpfunClient {
    pub(crate) fn build_common_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        if let Some(ref r) = self.referer {
            headers.insert(
                REFERER,
                HeaderValue::from_str(r)
                    .unwrap_or_else(|_| HeaderValue::from_static("https://simpfun.cn/")),
            );
        }
        headers
    }

    pub(crate) fn build_auth_headers(&self) -> Result<HeaderMap, SdkError> {
        let guard = self.token.load();
        let token = guard.as_ref().ok_or(SdkError::MissingToken)?;
        let mut headers = self.build_common_headers();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(token).unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        Ok(headers)
    }

    /// 发送请求并在遇到 429 时自动重试
    /// 
    /// # 安全性说明
    /// 
    /// 当前仅对 `429 Too Many Requests` 进行重试，POST 请求不会被重复发送。
    /// 如果未来扩展重试条件，需要注意非幂等请求的安全性。
    pub(crate) async fn send_with_retry<F>(&self, mut make: F) -> Result<reqwest::Response, SdkError>
    where
        F: FnMut() -> reqwest::RequestBuilder,
    {
        let mut attempt = 0u32;
        loop {
            let resp = make().send().await?;
            let status = resp.status();
            if status == StatusCode::TOO_MANY_REQUESTS && attempt < 3 {
                let wait = self.retry_delay(resp.headers(), attempt);
                sleep(wait).await;
                attempt += 1;
                continue;
            }
            return self.check_response(resp).await;
        }
    }

    fn retry_delay(&self, headers: &HeaderMap, attempt: u32) -> Duration {
        if let Some(value) = headers
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
        {
            return Duration::from_secs(value);
        }
        let base_ms = 500u64;
        let max_ms = 8000u64;
        let mult = match 1u64.checked_shl(attempt) {
            Some(v) => v,
            None => u64::MAX,
        };
        let ms = base_ms.saturating_mul(mult).min(max_ms);
        Duration::from_millis(ms)
    }

    async fn check_response(&self, resp: reqwest::Response) -> Result<reqwest::Response, SdkError> {
        let status = resp.status();
        if status.is_success() {
            Ok(resp)
        } else if status == StatusCode::UNAUTHORIZED {
            Err(SdkError::AuthFailed)
        } else {
            let body = resp.text().await.unwrap_or_else(|e| format!("无法读取响应体: {}", e));
            Err(SdkError::Status { status, body })
        }
    }

    pub(crate) async fn head_meta(&self, path: &str) -> Result<ResourceMeta, SdkError> {
        let url = format!("{}{}", self.base_url, path);
        let headers = self.build_auth_headers()?;
        let resp = self.send_with_retry(|| self.http.head(url.clone()).headers(headers.clone())).await?;
        let len = resp.content_length();
        let etag = resp.headers().get(ETAG).and_then(|v| v.to_str().ok()).map(|s| s.to_string());
        
        Ok(ResourceMeta { len, etag })
    }
}
