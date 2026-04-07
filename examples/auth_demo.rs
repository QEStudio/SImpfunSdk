//! 认证相关 API 示例
//!
//! 演示以下 API：
//! - user().login: 用户名密码登录
//! - user().login_and_set_token: 登录并保存 Token
//! - user().logout: 登出
//! - user().auth_info: 获取用户信息
//! - set_token / clear_token: Token 管理
//! - user().announcement_list: 公告列表
//! - user().point_history: 积分历史
//! - user().diamond_history: 钻石历史
//! - user().invite_info: 邀请信息

use simpfun::{SdkError, SimpfunClient};

#[tokio::main]
async fn main() -> Result<(), SdkError> {
    println!("=== 认证相关 API 示例 ===\n");

    let username = std::env::var("SIMPFUN_USERNAME").expect("请设置 SIMPFUN_USERNAME 环境变量");
    let password = std::env::var("SIMPFUN_PASSWORD").expect("请设置 SIMPFUN_PASSWORD 环境变量");

    let client = SimpfunClient::new()?;
    println!("客户端创建成功");

    println!("\n--- 方式一：user().login ---");
    let login_resp = client.user().login(&username, &password).await?;
    println!("登录成功！");
    println!("  响应码: {}", login_resp.code);
    println!("  消息: {}", login_resp.msg);
    println!(
        "  Token: {}...",
        login_resp
            .token
            .as_ref()
            .map(|t| &t[..20.min(t.len())])
            .unwrap_or("None")
    );

    if let Some(token) = &login_resp.token {
        client.set_token(token);
        println!("  Token 已设置");
    }

    println!("\n--- user().auth_info ---");
    let auth_info = client.user().auth_info().await?;
    println!("用户 ID: {}", auth_info.info.id);
    println!("用户名: {}", auth_info.info.username);
    println!("积分: {}", auth_info.info.point);
    println!("钻石: {}", auth_info.info.diamond);
    println!("已验证: {}", auth_info.info.verified);
    println!("开发者: {}", auth_info.info.is_dev);
    println!("Pro 用户: {}", auth_info.info.is_pro);

    println!("\n--- user().announcement_list ---");
    let announcements = client.user().announcement_list().await?;
    println!("公告数量: {}", announcements.list.len());
    for (i, ann) in announcements.list.iter().enumerate().take(3) {
        println!("  [{}] {} (ID: {})", i + 1, ann.title, ann.id);
    }

    println!("\n--- user().point_history ---");
    let point_history = client.user().point_history().await?;
    println!("积分记录数: {}", point_history.list.len());
    for record in point_history.list.iter().take(3) {
        println!(
            "  {} | 积分: {} | {}",
            record.create_time, record.point, record.comment
        );
    }

    println!("\n--- user().diamond_history ---");
    let diamond_history = client.user().diamond_history().await?;
    println!("钻石记录数: {}", diamond_history.list.len());
    for record in diamond_history.list.iter().take(3) {
        println!(
            "  {} | 钻石: {} | {}",
            record.create_time, record.diamond, record.comment
        );
    }

    println!("\n--- user().invite_info ---");
    let invite = client.user().invite_info().await?;
    println!("邀请码: {}", invite.data.invite_code);
    println!("注册次数: {}", invite.data.register_times);
    println!("总收入: {}", invite.data.register_total_income);

    println!("\n--- Token 管理 ---");
    println!(
        "当前 Token: {}...",
        client
            .token()
            .map(|t| t[..20.min(t.len())].to_string())
            .unwrap_or_default()
    );

    client.clear_token();
    println!("Token 已清除");
    println!("当前 Token: {:?}", client.token());

    println!("\n--- 方式二：user().login_and_set_token ---");
    let token = client.user().login_and_set_token(&username, &password).await?;
    println!("登录成功，Token: {}...", &token[..20.min(token.len())]);

    println!("\n--- user().logout ---");
    client.user().logout().await?;
    println!("已登出");

    println!("\n=== 示例结束 ===");
    Ok(())
}