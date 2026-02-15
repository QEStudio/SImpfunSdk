//! 实例管理相关 API

use crate::error::SdkError;
use crate::models::{
    InsListResponse,
    InsDetailResponse,
    WsInitResponse,
    PowerResponse,
    ChangeResponse,
    SupportResponse,
    SimpleMsgResponse,
    SftpResponse,
    TasksResponse,
    StatListResponse,
    DiamondPlanResponse,
};
use super::SimpfunClient;

impl SimpfunClient {
    /// 获取实例列表：`GET /api/ins/list`
    pub async fn ins_list(&self) -> Result<InsListResponse, SdkError> {
        let url = format!("{}/api/ins/list", self.base_url);
        let headers = self.build_auth_headers()?;

        let resp = self.send_with_retry(|| self.http.get(url.clone()).headers(headers.clone())).await?;
        let lr: InsListResponse = resp.json().await?;
        if lr.code != 200 {
            return Err(SdkError::Api {
                code: lr.code,
                msg: "获取实例列表失败".to_string(),
            });
        }
        Ok(lr)
    }

    /// 获取实例详情：`GET /api/ins/{id}/detail`
    pub async fn ins_detail(&self, id: i64) -> Result<InsDetailResponse, SdkError> {
        let url = format!("{}/api/ins/{}/detail", self.base_url, id);
        let headers = self.build_auth_headers()?;

        let resp = self.send_with_retry(|| self.http.get(url.clone()).headers(headers.clone())).await?;
        let ir: InsDetailResponse = resp.json().await?;
        if ir.code != 200 {
            return Err(SdkError::Api { code: ir.code, msg: "获取实例详情失败".to_string() });
        }
        Ok(ir)
    }

    /// 获取实例的 WS 连接信息（token 与 socket URL）
    pub async fn ins_ws_init(&self, id: i64) -> Result<WsInitResponse, SdkError> {
        let url = format!("{}/api/ins/{}/ws?", self.base_url, id);
        let headers = self.build_auth_headers()?;
        let resp = self.send_with_retry(|| self.http.get(url.clone()).headers(headers.clone())).await?;
        let wi: WsInitResponse = resp.json().await?;
        if wi.code != 200 {
            return Err(SdkError::Api { code: wi.code, msg: "获取WS信息失败".to_string() });
        }
        Ok(wi)
    }

    /// 控制实例电源：action 可为 `start`/`kill`/`stop` 等
    pub async fn ins_power(&self, id: i64, action: &str) -> Result<PowerResponse, SdkError> {
        let url = format!("{}/api/ins/{}/power?action={}", self.base_url, id, action);
        let headers = self.build_auth_headers()?;
        let resp = self.send_with_retry(|| self.http.get(url.clone()).headers(headers.clone())).await?;
        let pr: PowerResponse = resp.json().await?;
        if pr.code != 200 {
            return Err(SdkError::Api { code: pr.code, msg: pr.msg.clone() });
        }
        Ok(pr)
    }

    /// 重命名实例
    pub async fn ins_rename(&self, id: i64, name: &str) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("name", name)])?;
        let url = format!("{}/api/ins/{}/rename", self.base_url, id);
        let resp = self
            .send_with_retry(|| self.http.post(url.clone()).headers(headers.clone()).body(body.clone()))
            .await?;
        let sr: SimpleMsgResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api { code: sr.code, msg: sr.msg.clone() });
        }
        Ok(sr)
    }

    /// 更换实例套餐
    pub async fn ins_change(&self, id: i64, item_id: i64) -> Result<ChangeResponse, SdkError> {
        let headers = self.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("item_id", item_id.to_string())])?;
        let url = format!("{}/api/ins/{}/change", self.base_url, id);
        let resp = self
            .send_with_retry(|| self.http.post(url.clone()).headers(headers.clone()).body(body.clone()))
            .await?;
        let cr: ChangeResponse = resp.json().await?;
        if cr.code != 200 {
            return Err(SdkError::Api { code: cr.code, msg: cr.msg.clone() });
        }
        Ok(cr)
    }

    /// 设置默认端口分配
    pub async fn ins_allocation_default(&self, id: i64, port_id: i64) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("port_id", port_id.to_string())])?;
        let url = format!("{}/api/ins/{}/allocation", self.base_url, id);
        let resp = self
            .send_with_retry(|| self.http.put(url.clone()).headers(headers.clone()).body(body.clone()))
            .await?;
        let sr: SimpleMsgResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api { code: sr.code, msg: sr.msg.clone() });
        }
        Ok(sr)
    }

    /// 获取技术支持信息
    pub async fn ins_support_info(&self, id: i64) -> Result<SupportResponse, SdkError> {
        let headers = self.build_auth_headers()?;
        let url = format!("{}/api/ins/{}/support?", self.base_url, id);
        let resp = self.send_with_retry(|| self.http.get(url.clone()).headers(headers.clone())).await?;
        let sr: SupportResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api { code: sr.code, msg: "获取技术支持信息失败".to_string() });
        }
        Ok(sr)
    }

    /// 创建技术支持请求
    pub async fn ins_support_create(&self, id: i64, comment: &str) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("comment", comment)])?;
        let url = format!("{}/api/ins/{}/support", self.base_url, id);
        let resp = self
            .send_with_retry(|| self.http.post(url.clone()).headers(headers.clone()).body(body.clone()))
            .await?;
        let sr: SimpleMsgResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api { code: sr.code, msg: sr.msg.clone() });
        }
        Ok(sr)
    }

    /// 结束技术支持
    pub async fn ins_support_end(&self, id: i64, feedback: &str) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.build_auth_headers()?;
        let body = serde_urlencoded::to_string([("feedback", feedback)])?;
        let url = format!("{}/api/ins/{}/support", self.base_url, id);
        let resp = self
            .send_with_retry(|| self.http.delete(url.clone()).headers(headers.clone()).body(body.clone()))
            .await?;
        let sr: SimpleMsgResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api { code: sr.code, msg: sr.msg.clone() });
        }
        Ok(sr)
    }

    /// 获取实例钻石套餐
    pub async fn ins_diamond_plan(&self, id: i64) -> Result<DiamondPlanResponse, SdkError> {
        let headers = self.build_auth_headers()?;
        let url = format!("{}/api/ins/{}/diamond_plan?", self.base_url, id);
        let resp = self.send_with_retry(|| self.http.get(url.clone()).headers(headers.clone())).await?;
        let dr: DiamondPlanResponse = resp.json().await?;
        if dr.code != 200 {
            return Err(SdkError::Api { code: dr.code, msg: "获取钻石套餐失败".to_string() });
        }
        Ok(dr)
    }

    /// 获取 SFTP 信息
    pub async fn ins_sftp(&self, id: i64) -> Result<SftpResponse, SdkError> {
        let headers = self.build_auth_headers()?;
        let url = format!("{}/api/ins/{}/sftp?", self.base_url, id);
        let resp = self.send_with_retry(|| self.http.get(url.clone()).headers(headers.clone())).await?;
        let sr: SftpResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api { code: sr.code, msg: "获取SFTP信息失败".to_string() });
        }
        Ok(sr)
    }

    /// 获取实例任务列表
    pub async fn ins_tasks(&self, id: i64) -> Result<TasksResponse, SdkError> {
        let headers = self.build_auth_headers()?;
        let url = format!("{}/api/ins/{}/tasks?", self.base_url, id);
        let resp = self.send_with_retry(|| self.http.get(url.clone()).headers(headers.clone())).await?;
        let tr: TasksResponse = resp.json().await?;
        if tr.code != 200 {
            return Err(SdkError::Api { code: tr.code, msg: "获取任务列表失败".to_string() });
        }
        Ok(tr)
    }

    /// 获取实例历史统计
    pub async fn ins_stat(&self, id: i64) -> Result<StatListResponse, SdkError> {
        let headers = self.build_auth_headers()?;
        let url = format!("{}/api/ins/{}/stat?", self.base_url, id);
        let resp = self.send_with_retry(|| self.http.get(url.clone()).headers(headers.clone())).await?;
        let sr: StatListResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api { code: sr.code, msg: "获取历史统计失败".to_string() });
        }
        Ok(sr)
    }

    /// 重装实例
    pub async fn ins_reinstall(
        &self,
        id: i64,
        version_id: i64,
        diff: bool,
        save: bool,
        custom: bool,
    ) -> Result<SimpleMsgResponse, SdkError> {
        let headers = self.build_auth_headers()?;
        let mut params = vec![
            ("version_id", version_id.to_string()),
            ("diff", diff.to_string()),
            ("save", save.to_string()),
        ];
        if custom {
            params.push(("custom", "true".to_string()));
        }
        let body = serde_urlencoded::to_string(params)?;
        let url = format!("{}/api/ins/{}/reinstall", self.base_url, id);
        let resp = self
            .send_with_retry(|| self.http.post(url.clone()).headers(headers.clone()).body(body.clone()))
            .await?;
        let sr: SimpleMsgResponse = resp.json().await?;
        if sr.code != 200 {
            return Err(SdkError::Api { code: sr.code, msg: sr.msg.clone() });
        }
        Ok(sr)
    }
}
