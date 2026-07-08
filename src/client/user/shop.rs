//! 镜像类别相关 API

use super::UserClient;
use crate::error::SdkError;
use crate::models::ShopListResponse;

impl<'a> UserClient<'a> {
    /// 获取镜像类别列表
    pub async fn shop_list(&self, version_id: i64) -> Result<ShopListResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let qs = serde_urlencoded::to_string([("version_id", version_id)])?;
        let url = format!("{}/api/shop/list?{}", self.inner.base_url, qs);
        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;
        let sr: ShopListResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api {
                code: sr.code,
                msg: "获取镜像类别失败".to_string(),
            });
        }
        Ok(sr)
    }
}
