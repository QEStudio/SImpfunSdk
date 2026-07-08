//! 备份与回滚相关 API

use super::UserClient;
use crate::error::SdkError;
use crate::models::{
    BackupDownloadResponse, BackupListResponse, RollbackListResponse, SimpleMsgResponse,
};

impl<'a> UserClient<'a> {
    /// 获取备份列表
    pub async fn ins_backup_list(&self, id: i64) -> Result<BackupListResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let url = format!("{}/api/ins/{}/backup?", self.inner.base_url, id);

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let br: BackupListResponse = resp.json().await?;
        if br.code != 200 {
            return Err(SdkError::Api {
                code: br.code,
                msg: "获取备份列表失败".to_string(),
            });
        }
        Ok(br)
    }

    /// 创建备份
    pub async fn ins_backup_create(
        &self,
        id: i64,
        tag: &str,
    ) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("tag", tag)])?;
        let url = format!("{}/api/ins/{}/backup", self.inner.base_url, id);

        let resp = self
            .inner
            .send_with_retry(|| {
                self.inner
                    .http
                    .post(url.clone())
                    .headers(headers.clone())
                    .body(body.clone())
            })
            .await?;

        let sr: SimpleMsgResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api {
                code: sr.code,
                msg: sr.msg.clone(),
            });
        }
        Ok(sr)
    }

    /// 重命名备份
    pub async fn ins_backup_rename(
        &self,
        id: i64,
        backup_id: i64,
        new_tag: &str,
    ) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let body = serde_urlencoded::to_string([
            ("backup_id", backup_id.to_string()),
            ("new_tag", new_tag.to_string()),
        ])?;
        let url = format!("{}/api/ins/{}/backup", self.inner.base_url, id);

        let resp = self
            .inner
            .send_with_retry(|| {
                self.inner
                    .http
                    .put(url.clone())
                    .headers(headers.clone())
                    .body(body.clone())
            })
            .await?;

        let sr: SimpleMsgResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api {
                code: sr.code,
                msg: sr.msg.clone(),
            });
        }
        Ok(sr)
    }

    /// 恢复备份
    pub async fn ins_backup_restore(
        &self,
        id: i64,
        backup_id: i64,
    ) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("backup_id", backup_id.to_string())])?;
        let url = format!("{}/api/ins/{}/backup", self.inner.base_url, id);

        let resp = self
            .inner
            .send_with_retry(|| {
                self.inner
                    .http
                    .patch(url.clone())
                    .headers(headers.clone())
                    .body(body.clone())
            })
            .await?;

        let sr: SimpleMsgResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api {
                code: sr.code,
                msg: sr.msg.clone(),
            });
        }
        Ok(sr)
    }

    /// 删除备份
    pub async fn ins_backup_delete(
        &self,
        id: i64,
        backup_id: i64,
    ) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("backup_id", backup_id.to_string())])?;
        let url = format!("{}/api/ins/{}/backup", self.inner.base_url, id);

        let resp = self
            .inner
            .send_with_retry(|| {
                self.inner
                    .http
                    .delete(url.clone())
                    .headers(headers.clone())
                    .body(body.clone())
            })
            .await?;

        let sr: SimpleMsgResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api {
                code: sr.code,
                msg: sr.msg.clone(),
            });
        }
        Ok(sr)
    }

    /// 获取备份下载链接
    pub async fn ins_backup_download(
        &self,
        id: i64,
        backup_id: i64,
    ) -> Result<BackupDownloadResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let qs = serde_urlencoded::to_string([("down_id", backup_id.to_string())])?;
        let url = format!("{}/api/ins/{}/backup?{}", self.inner.base_url, id, qs);

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let br: BackupDownloadResponse = resp.json().await?;
        if br.code != 200 {
            return Err(SdkError::Api {
                code: br.code,
                msg: "获取备份下载链接失败".to_string(),
            });
        }
        Ok(br)
    }

    /// 获取回滚时间点列表
    pub async fn ins_rollback_points(&self, id: i64) -> Result<RollbackListResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let url = format!("{}/api/ins/{}/rollback?", self.inner.base_url, id);

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let rr: RollbackListResponse = resp.json().await?;
        if rr.code != 200 {
            return Err(SdkError::Api {
                code: rr.code,
                msg: "获取回滚时间失败".to_string(),
            });
        }
        Ok(rr)
    }

    /// 执行回滚
    pub async fn ins_rollback_create(
        &self,
        id: i64,
        rollback_time: &str,
    ) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("rollback_time", rollback_time)])?;
        let url = format!("{}/api/ins/{}/rollback", self.inner.base_url, id);

        let resp = self
            .inner
            .send_with_retry(|| {
                self.inner
                    .http
                    .post(url.clone())
                    .headers(headers.clone())
                    .body(body.clone())
            })
            .await?;

        let sr: SimpleMsgResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api {
                code: sr.code,
                msg: sr.msg.clone(),
            });
        }
        Ok(sr)
    }
}
