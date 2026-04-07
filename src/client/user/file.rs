//! 文件操作相关 API

use crate::error::SdkError;
use crate::models::{
    InsFileListResponse,
    InsFileContentResponse,
    SimpleMsgResponse,
};
use super::UserClient;

impl<'a> UserClient<'a> {
    /// 获取文件列表
    pub async fn ins_file_list(&self, id: i64, path: &str) -> Result<InsFileListResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let qs = serde_urlencoded::to_string([("path", path)])?;
        let url = format!("{}/api/ins/{}/file/list?{}", self.inner.base_url, id, qs);

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let fr: InsFileListResponse = resp.json().await?;
        if fr.code != 200 {
            return Err(SdkError::Api {
                code: fr.code,
                msg: "获取文件列表失败".to_string(),
            });
        }
        Ok(fr)
    }

    /// 获取文件内容
    pub async fn ins_file_fetch(&self, id: i64, path: &str) -> Result<InsFileContentResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let qs = serde_urlencoded::to_string([("path", path)])?;
        let url = format!("{}/api/ins/{}/file/fetch?{}", self.inner.base_url, id, qs);

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let cr: InsFileContentResponse = resp.json().await?;
        if cr.code != 200 {
            return Err(SdkError::Api {
                code: cr.code,
                msg: "获取文件内容失败".to_string(),
            });
        }
        Ok(cr)
    }

    /// 保存文件
    pub async fn ins_file_save(&self, id: i64, path: &str, content: &str) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("path", path), ("content", content)])?;
        let url = format!("{}/api/ins/{}/file/save", self.inner.base_url, id);

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

    /// 创建文件或目录
    pub async fn ins_file_create(&self, id: i64, mode: &str, root: &str, name: &str) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("mode", mode), ("root", root), ("name", name)])?;
        let url = format!("{}/api/ins/{}/file/create", self.inner.base_url, id);

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

    /// 重命名文件
    pub async fn ins_file_rename(&self, id: i64, origin: &str, target: &str) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("origin", origin), ("target", target)])?;
        let url = format!("{}/api/ins/{}/file/rename", self.inner.base_url, id);

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

    /// 删除文件
    pub async fn ins_file_delete(&self, id: i64, list: Vec<String>) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let list_json = serde_json::to_string(&list)?;
        let body = serde_urlencoded::to_string([("list", list_json)])?;
        let url = format!("{}/api/ins/{}/file/delete", self.inner.base_url, id);

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

    /// 压缩文件
    pub async fn ins_file_archive(
        &self,
        id: i64,
        root: &str,
        files: Vec<String>,
        format: &str,
    ) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let files_json = serde_json::to_string(&files)?;
        let body = serde_urlencoded::to_string([
            ("root", root),
            ("files", files_json.as_str()),
            ("format", format),
        ])?;
        let url = format!("{}/api/ins/{}/file/archive", self.inner.base_url, id);

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

    /// 解压文件
    pub async fn ins_file_unarchive(
        &self,
        id: i64,
        root: &str,
        file: &str,
    ) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("root", root), ("file", file)])?;
        let url = format!("{}/api/ins/{}/file/unarchive", self.inner.base_url, id);

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

    /// 粘贴文件
    pub async fn ins_file_paste(
        &self,
        id: i64,
        list: Vec<String>,
        target: &str,
    ) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let list_json = serde_json::to_string(&list)?;
        let body = serde_urlencoded::to_string([
            ("list", list_json),
            ("target", target.to_string()),
        ])?;
        let url = format!("{}/api/ins/{}/file/paste", self.inner.base_url, id);

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

    /// 复制文件
    pub async fn ins_file_copy(&self, id: i64, location: &str) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("location", location)])?;
        let url = format!("{}/api/ins/{}/file/copy", self.inner.base_url, id);

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