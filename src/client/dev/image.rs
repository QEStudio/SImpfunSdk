use crate::error::SdkError;
use super::DevClient;
use crate::models::{
    DevImageDetail,
    DevImageDetailResponse,
    DevImageFeedback,
    DevImageFeedbackResponse,
    DevImageItem,
    DevImageListResponse,
    DevImageVersion,
    DevImageVersionResponse,
};

impl<'a> DevClient<'a> {
    /// 获取开发者镜像列表
    pub async fn image_list(&self) -> Result<Vec<DevImageItem>, SdkError> {
        let url = format!("{}/api/dev/list", self.inner.base_url);
        let headers = self.inner.build_auth_headers()?;

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let data: DevImageListResponse = resp.json().await?;
        if data.code != 200 {
            return Err(SdkError::Api {
                code: data.code,
                msg: data.msg.unwrap_or_else(|| "获取开发者镜像列表失败".to_string()),
            });
        }

        Ok(data.list.unwrap_or_default())
    }

    /// 获取镜像详情
    pub async fn image_detail(&self, id: i64) -> Result<DevImageDetail, SdkError> {
        let url = format!("{}/api/dev/{}/detail", self.inner.base_url, id);
        let headers = self.inner.build_auth_headers()?;

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let data: DevImageDetailResponse = resp.json().await?;
        if data.code != 200 {
            return Err(SdkError::Api {
                code: data.code,
                msg: data.msg.unwrap_or_else(|| format!("获取镜像 {} 详情失败", id)),
            });
        }

        data.data.ok_or_else(|| SdkError::Api {
            code: data.code,
            msg: format!("镜像 {} 详情为空", id),
        })
    }

    /// 获取镜像版本列表
    pub async fn image_versions(&self, id: i64) -> Result<Vec<DevImageVersion>, SdkError> {
        let url = format!("{}/api/dev/{}/version", self.inner.base_url, id);
        let headers = self.inner.build_auth_headers()?;

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let data: DevImageVersionResponse = resp.json().await?;
        if data.code != 200 {
            return Err(SdkError::Api {
                code: data.code,
                msg: data.msg.unwrap_or_else(|| format!("获取镜像 {} 版本失败", id)),
            });
        }

        Ok(data.list.unwrap_or_default())
    }

    /// 获取镜像反馈
    pub async fn image_feedback(&self, id: i64) -> Result<Vec<DevImageFeedback>, SdkError> {
        let url = format!("{}/api/dev/{}/feedback", self.inner.base_url, id);
        let headers = self.inner.build_auth_headers()?;

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let data: DevImageFeedbackResponse = resp.json().await?;
        if data.code != 200 {
            return Err(SdkError::Api {
                code: data.code,
                msg: data.msg.unwrap_or_else(|| format!("获取镜像 {} 反馈失败", id)),
            });
        }

        Ok(data.list.unwrap_or_default())
    }
}