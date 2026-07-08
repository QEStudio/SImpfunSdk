use super::DevClient;
use crate::error::SdkError;
use crate::models::{DevSupport, DevSupportListResponse};

impl<'a> DevClient<'a> {
    /// 获取技术支持列表
    pub async fn support_list(&self) -> Result<Vec<DevSupport>, SdkError> {
        let url = format!("{}/api/dev/support/list", self.inner.base_url);
        let headers = self.inner.build_auth_headers()?;

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let data: DevSupportListResponse = resp.json().await?;
        if data.code != 200 {
            return Err(SdkError::Api {
                code: data.code,
                msg: "获取技术支持列表失败".to_string(),
            });
        }

        Ok(data.list)
    }
}
