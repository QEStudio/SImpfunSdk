//! 用户认证相关 API

use crate::error::SdkError;
use crate::models::{
    AuthInfoResponse,
    LoginResponse,
    AnnouncementListResponse,
    PointHistoryResponse,
    DiamondHistoryResponse,
    InviteResponse,
};
use super::UserClient;

impl<'a> UserClient<'a> {
    /// 登录接口：`POST /api/auth/login`
    pub async fn login(&self, username: &str, passwd: &str) -> Result<LoginResponse, SdkError> {
        let url = format!("{}/api/auth/login", self.inner.base_url);

        let form = serde_urlencoded::to_string([("username", username), ("passwd", passwd)])?;
        let headers = self.inner.build_common_headers();

        let resp = self
            .inner
            .send_with_retry(|| {
                self.inner
                    .http
                    .post(url.clone())
                    .headers(headers.clone())
                    .body(form.clone())
            })
            .await?;

        let lr: LoginResponse = resp.json().await?;
        if lr.code != 200 {
            return Err(SdkError::Api {
                code: lr.code,
                msg: lr.msg.clone(),
            });
        }
        Ok(lr)
    }

    /// 登录并在客户端中保存令牌
    pub async fn login_and_set_token(
        &self,
        username: &str,
        passwd: &str,
    ) -> Result<String, SdkError> {
        let res = self.login(username, passwd).await?;
        if let Some(tok) = res.token.clone() {
            self.inner.set_token(tok.clone());
            Ok(tok)
        } else {
            Err(SdkError::Api {
                code: res.code,
                msg: "登录响应中缺少token".to_string(),
            })
        }
    }

    /// 登出接口：`POST /api/auth/logout`
    pub async fn logout(&self) -> Result<(), SdkError> {
        let url = format!("{}/api/auth/logout", self.inner.base_url);
        let headers = self.inner.build_auth_headers()?;

        self.inner
            .send_with_retry(|| {
                self.inner
                    .http
                    .post(url.clone())
                    .headers(headers.clone())
                    .body("")
            })
            .await?;
        Ok(())
    }

    /// 获取用户信息：`GET /api/auth/info`
    pub async fn auth_info(&self) -> Result<AuthInfoResponse, SdkError> {
        let url = format!("{}/api/auth/info", self.inner.base_url);
        let headers = self.inner.build_auth_headers()?;

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;
        let ir: AuthInfoResponse = resp.json().await?;
        if ir.code != 200 {
            return Err(SdkError::Api {
                code: ir.code,
                msg: "获取用户信息失败".to_string(),
            });
        }
        Ok(ir)
    }

    /// 获取公告列表：`GET /api/announcement`
    pub async fn announcement_list(&self) -> Result<AnnouncementListResponse, SdkError> {
        let url = format!("{}/api/announcement", self.inner.base_url);
        let headers = self.inner.build_auth_headers()?;

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;
        let ar: AnnouncementListResponse = resp.json().await?;
        if ar.code != 200 {
            return Err(SdkError::Api {
                code: ar.code,
                msg: "获取公告列表失败".to_string(),
            });
        }
        Ok(ar)
    }

    /// 标记公告为已读：`POST /api/announcement_read`
    pub async fn announcement_read(&self) -> Result<(), SdkError> {
        let url = format!("{}/api/announcement_read", self.inner.base_url);
        let headers = self.inner.build_auth_headers()?;

        let _resp = self
            .inner
            .send_with_retry(|| {
                self.inner
                    .http
                    .post(url.clone())
                    .headers(headers.clone())
                    .body("")
            })
            .await?;

        Ok(())
    }

    /// 获取积分变动历史：`GET /api/pointhistory`
    pub async fn point_history(&self) -> Result<PointHistoryResponse, SdkError> {
        let url = format!("{}/api/pointhistory", self.inner.base_url);
        let headers = self.inner.build_auth_headers()?;

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;
        let pr: PointHistoryResponse = resp.json().await?;
        if pr.code != 200 {
            return Err(SdkError::Api {
                code: pr.code,
                msg: "获取积分历史失败".to_string(),
            });
        }
        Ok(pr)
    }

    /// 获取钻石变动历史：`GET /api/diamondhistory`
    pub async fn diamond_history(&self) -> Result<DiamondHistoryResponse, SdkError> {
        let url = format!("{}/api/diamondhistory", self.inner.base_url);
        let headers = self.inner.build_auth_headers()?;

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;
        let dr: DiamondHistoryResponse = resp.json().await?;
        if dr.code != 200 {
            return Err(SdkError::Api {
                code: dr.code,
                msg: "获取钻石历史失败".to_string(),
            });
        }
        Ok(dr)
    }

    /// 获取邀请信息：`GET /api/invite`
    pub async fn invite_info(&self) -> Result<InviteResponse, SdkError> {
        let url = format!("{}/api/invite", self.inner.base_url);
        let headers = self.inner.build_auth_headers()?;

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;
        let ir: InviteResponse = resp.json().await?;
        if ir.code != 200 {
            return Err(SdkError::Api {
                code: ir.code,
                msg: "获取邀请信息失败".to_string(),
            });
        }
        Ok(ir)
    }
}