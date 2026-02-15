//! 商店相关 API

use crate::error::SdkError;
use crate::models::ShopListResponse;
use super::SimpfunClient;

impl SimpfunClient {
    /// 获取商店套餐列表
    pub async fn shop_list(&self, version_id: i64) -> Result<ShopListResponse, SdkError> {
        let headers = self.build_auth_headers()?;
        let qs = serde_urlencoded::to_string([("version_id", version_id)])?;
        let url = format!("{}/api/shop/list?{}", self.base_url, qs);
        let resp = self.send_with_retry(|| self.http.get(url.clone()).headers(headers.clone())).await?;
        let sr: ShopListResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api { code: sr.code, msg: "获取商店套餐失败".to_string() });
        }
        Ok(sr)
    }
}
