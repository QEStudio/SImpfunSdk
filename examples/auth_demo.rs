//! 认证相关 API 示例
//! 
//! 演示以下 API：
//! - login: 用户名密码登录
//! - login_and_set_token: 登录并保存 Token
//! - logout: 登出
//! - auth_info: 获取用户信息
//! - set_token / clear_token: Token 管理
//! - announcement_list: 公告列表
//! - point_history: 积分历史
//! - diamond_history: 钻石历史
//! - invite_info: 邀请信息

use simpfun_sdk::{SimpfunClient, SdkError};

#[tokio::main]
async fn main() -> Result<(), SdkError> {
    println!("=== 认证相关 API 示例 ===\n");

    // 从环境变量获取凭据
    let username = std::env::var("SIMPFUN_USERNAME")
        .expect("请设置 SIMPFUN_USERNAME 环境变量");
    let password = std::env::var("SIMPFUN_PASSWORD")
        .expect("请设置 SIMPFUN_PASSWORD 环境变量");

    // 1. 创建客户端
    let client = SimpfunClient::new()?;
    println!("客户端创建成功");

    // 2. 登录（方式一：手动处理 Token）
    println!("\n--- 方式一：login ---");
    let login_resp = client.login(&username, &password).await?;
    println!("登录成功！");
    println!("  响应码: {}", login_resp.code);
    println!("  消息: {}", login_resp.msg);
    println!("  Token: {}...", login_resp.token.as_ref().map(|t| &t[..20.min(t.len())]).unwrap_or("None"));
    
    // 手动设置 Token
    if let Some(token) = &login_resp.token {
        client.set_token(token);
        println!("  Token 已设置");
    }

    // 3. 获取用户信息
    println!("\n--- auth_info ---");
    let auth_info = client.auth_info().await?;
    println!("用户 ID: {}", auth_info.info.id);
    println!("用户名: {}", auth_info.info.username);
    println!("积分: {}", auth_info.info.point);
    println!("钻石: {}", auth_info.info.diamond);
    println!("已验证: {}", auth_info.info.verified);
    println!("开发者: {}", auth_info.info.is_dev);
    println!("Pro 用户: {}", auth_info.info.is_pro);

    // 4. 公告列表
    println!("\n--- announcement_list ---");
    let announcements = client.announcement_list().await?;
    println!("公告数量: {}", announcements.list.len());
    for (i, ann) in announcements.list.iter().enumerate().take(3) {
        println!("  [{}] {} (ID: {})", i + 1, ann.title, ann.id);
    }

    // 5. 积分历史
    println!("\n--- point_history ---");
    let point_history = client.point_history().await?;
    println!("积分记录数: {}", point_history.list.len());
    for record in point_history.list.iter().take(3) {
        println!("  {} | 积分: {} | {}", record.create_time, record.point, record.comment);
    }

    // 6. 钻石历史
    println!("\n--- diamond_history ---");
    let diamond_history = client.diamond_history().await?;
    println!("钻石记录数: {}", diamond_history.list.len());
    for record in diamond_history.list.iter().take(3) {
        println!("  {} | 钻石: {} | {}", record.create_time, record.diamond, record.comment);
    }

    // 7. 邀请信息
    println!("\n--- invite_info ---");
    let invite = client.invite_info().await?;
    println!("邀请码: {}", invite.data.invite_code);
    println!("注册次数: {}", invite.data.register_times);
    println!("总收入: {}", invite.data.register_total_income);

    // 8. Token 管理
    println!("\n--- Token 管理 ---");
    println!("当前 Token: {}...", client.token().map(|t| t[..20.min(t.len())].to_string()).unwrap_or_default());
    
    client.clear_token();
    println!("Token 已清除");
    println!("当前 Token: {:?}", client.token());

    // 9. 登录（方式二：自动保存 Token）
    println!("\n--- 方式二：login_and_set_token ---");
    let token = client.login_and_set_token(&username, &password).await?;
    println!("登录成功，Token: {}...", &token[..20.min(token.len())]);

    // 10. 登出
    println!("\n--- logout ---");
    client.logout().await?;
    println!("已登出");

    println!("\n=== 示例结束 ===");
    Ok(())
}
