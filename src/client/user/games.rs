//! 游戏与镜像相关 API

use super::UserClient;
use crate::error::SdkError;
use crate::models::{
    CustomGameListResponse, CustomVersionListResponse, GameKindListResponse, GameListResponse,
    GameVersionListResponse,
};

impl<'a> UserClient<'a> {
    /// 获取游戏列表
    pub async fn games_list(&self, custom: bool) -> Result<GameListResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let qs = if custom {
            serde_urlencoded::to_string([("custom", "true")])?
        } else {
            String::new()
        };

        let url = if qs.is_empty() {
            format!("{}/api/games/list?", self.inner.base_url)
        } else {
            format!("{}/api/games/list?{}", self.inner.base_url, qs)
        };

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let gr: GameListResponse = resp.json().await?;
        if gr.code != 200 {
            return Err(SdkError::Api {
                code: gr.code,
                msg: "获取游戏列表失败".to_string(),
            });
        }
        Ok(gr)
    }

    /// 获取游戏种类列表
    pub async fn games_kind_list(
        &self,
        game_id: i64,
        from_version_id: i64,
    ) -> Result<GameKindListResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let qs = serde_urlencoded::to_string([
            ("game_id", game_id.to_string()),
            ("from_version_id", from_version_id.to_string()),
        ])?;
        let url = format!("{}/api/games/kindlist?{}", self.inner.base_url, qs);

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let gr: GameKindListResponse = resp.json().await?;
        if gr.code != 200 {
            return Err(SdkError::Api {
                code: gr.code,
                msg: "获取游戏种类列表失败".to_string(),
            });
        }
        Ok(gr)
    }

    /// 获取游戏版本列表
    pub async fn games_version_list(
        &self,
        kind_id: i64,
    ) -> Result<GameVersionListResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let qs = serde_urlencoded::to_string([("kind_id", kind_id.to_string())])?;
        let url = format!("{}/api/games/versionlist?{}", self.inner.base_url, qs);

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let gr: GameVersionListResponse = resp.json().await?;
        if gr.code != 200 {
            return Err(SdkError::Api {
                code: gr.code,
                msg: "获取游戏版本列表失败".to_string(),
            });
        }
        Ok(gr)
    }

    /// 获取游戏列表(第三方镜像)
    pub async fn games_custom_list(
        &self,
        game_id: i64,
        from_version_id: i64,
    ) -> Result<CustomGameListResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let qs = serde_urlencoded::to_string([
            ("game_id", game_id.to_string()),
            ("from_version_id", from_version_id.to_string()),
        ])?;
        let url = format!("{}/api/games/customlist?{}", self.inner.base_url, qs);

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let gr: CustomGameListResponse = resp.json().await?;
        if gr.code != 200 {
            return Err(SdkError::Api {
                code: gr.code,
                msg: "获取第三方镜像列表失败".to_string(),
            });
        }
        Ok(gr)
    }

    /// 获取游戏版本列表(第三方镜像)
    pub async fn games_custom_version_list(
        &self,
        kind_id: i64,
    ) -> Result<CustomVersionListResponse, SdkError> {
        let headers = self.inner.build_auth_headers()?;
        let qs = serde_urlencoded::to_string([("kind_id", kind_id.to_string())])?;
        let url = format!(
            "{}/api/games/custom_versionlist?{}",
            self.inner.base_url, qs
        );

        let resp = self
            .inner
            .send_with_retry(|| self.inner.http.get(url.clone()).headers(headers.clone()))
            .await?;

        let gr: CustomVersionListResponse = resp.json().await?;
        if gr.code != 200 {
            return Err(SdkError::Api {
                code: gr.code,
                msg: "获取第三方版本列表失败".to_string(),
            });
        }
        Ok(gr)
    }
}
