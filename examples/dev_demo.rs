//! 开发者 API 示例
//!
//! 演示以下 API：
//! - dev().image_list: 开发者镜像列表
//! - dev().image_detail: 镜像详情
//! - dev().image_versions: 镜像版本列表
//! - dev().image_feedback: 镜像反馈列表
//!
//! 注意：
//! - 开发者 API 需要开发者账户
//! - 登录仍然属于 user()，因为开发者本质上也是平台用户

use simpfun::{SdkError, SimpfunClient};

fn get_credentials() -> (String, String) {
    let username =
        std::env::var("SIMPFUN_USERNAME").expect("请设置 SIMPFUN_USERNAME 环境变量");
    let password =
        std::env::var("SIMPFUN_PASSWORD").expect("请设置 SIMPFUN_PASSWORD 环境变量");
    (username, password)
}

#[tokio::main]
async fn main() -> Result<(), SdkError> {
    println!("=== 开发者 API 示例 ===\n");

    let (username, password) = get_credentials();

    // 1. 创建客户端并登录
    let client = SimpfunClient::new()?;
    client
        .user()
        .login_and_set_token(&username, &password)
        .await?;
    println!("登录成功\n");

    // 2. 先确认当前账户是不是开发者
    println!("--- user().auth_info ---");
    let me = client.user().auth_info().await?;
    println!("当前用户: {}", me.info.username);
    println!("开发者: {}", me.info.is_dev);

    if !me.info.is_dev {
        println!("\n当前账号不是开发者账号，无法继续演示 dev() API");
        return Ok(());
    }

    // 3. 开发者镜像列表
    println!("\n--- dev().image_list ---");
    let images = client.dev().image_list().await?;
    println!("镜像数量: {}", images.len());

    for image in images.iter().take(10) {
        println!(
            "  [{}] {} | 公开状态: {:?}",
            image.id, image.name, image.state
        );
    }

    let first_id = match images.first().map(|i| i.id) {
        Some(id) => id,
        None => {
            println!("\n没有开发者镜像，示例结束");
            return Ok(());
        }
    };

    // 4. 镜像详情
    println!("\n--- dev().image_detail (id={}) ---", first_id);
    let detail = client.dev().image_detail(first_id).await?;
    println!("ID: {:?}", detail.id);
    println!("名称: {:?}", detail.name);
    println!("公开: {}", detail.is_public());
    println!("安装数: {}", detail.installations());
    println!("赞: {:?}", detail.like);
    println!("踩: {:?}", detail.dislike);

    // 5. 镜像版本
    println!("\n--- dev().image_versions (id={}) ---", first_id);
    let versions = client.dev().image_versions(first_id).await?;
    println!("版本数量: {}", versions.len());
    for v in versions.iter().take(5) {
        println!("  id={:?}, public={:?}", v.id, v.state);
    }

    // 6. 镜像反馈
    println!("\n--- dev().image_feedback (id={}) ---", first_id);
    let feedback = client.dev().image_feedback(first_id).await?;
    println!("反馈数量: {}", feedback.len());
    for f in feedback.iter().take(5) {
        println!("  id={:?}", f.id);
    }

    println!("\n=== 示例结束 ===");
    Ok(())
}