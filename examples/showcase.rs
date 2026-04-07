//! 交互式 Showcase / CLI 示例
//!
//! 说明：
//! - 这是一个“示例型交互客户端”
//! - 目标是尽量覆盖 SDK 中的大部分 API
//! - 修改型 API 默认不直接暴露，避免误操作
//! - 主要适合本地手工测试和体验 SDK

use simpfun_sdk::{
    connect_ins_ws, Event, EventManager, SdkError, SimpfunClient, WsEvent,
};

use tokio::time::{sleep, Duration};

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = std::io::Write::flush(&mut std::io::stdout());

    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    buf.trim().to_string()
}

fn read_i64(prompt: &str) -> Option<i64> {
    read_line(prompt).parse::<i64>().ok()
}

async fn login_client() -> Result<SimpfunClient, SdkError> {
    let client = SimpfunClient::new()?;

    if let Ok(token) = std::env::var("SIMPFUN_TOKEN") {
        client.set_token(token);
        println!("使用环境变量 Token 登录成功");
        return Ok(client);
    }

    let username = std::env::var("SIMPFUN_USERNAME")
        .unwrap_or_else(|_| read_line("请输入用户名: "));
    let password = std::env::var("SIMPFUN_PASSWORD")
        .unwrap_or_else(|_| read_line("请输入密码: "));

    client.user().login_and_set_token(&username, &password).await?;
    println!("用户名密码登录成功");

    Ok(client)
}

fn print_main_menu() {
    println!();
    println!("==================== 主菜单 ====================");
    println!("1. 用户 / 认证 API");
    println!("2. 实例 API");
    println!("3. 文件 API");
    println!("4. 备份 / 回滚 API");
    println!("5. 商店 / 游戏 API");
    println!("6. 开发者 API");
    println!("7. WebSocket 演示");
    println!("8. Event 轮询演示");
    println!("9. 显示当前 Token");
    println!("0. 退出");
    println!("===============================================");
}

fn print_auth_menu() {
    println!();
    println!("--- 用户 / 认证 API ---");
    println!("1. auth_info");
    println!("2. announcement_list");
    println!("3. point_history");
    println!("4. diamond_history");
    println!("5. invite_info");
    println!("6. logout");
    println!("0. 返回");
}

fn print_instance_menu() {
    println!();
    println!("--- 实例 API ---");
    println!("1. ins_list");
    println!("2. ins_detail");
    println!("3. ins_ws_init");
    println!("4. ins_sftp");
    println!("5. ins_tasks");
    println!("6. ins_stat");
    println!("7. ins_diamond_plan");
    println!("8. ins_support_info");
    println!("0. 返回");
}

fn print_file_menu() {
    println!();
    println!("--- 文件 API ---");
    println!("1. ins_file_list");
    println!("2. ins_file_fetch");
    println!("0. 返回");
}

fn print_backup_menu() {
    println!();
    println!("--- 备份 / 回滚 API ---");
    println!("1. ins_backup_list");
    println!("2. ins_backup_download");
    println!("3. ins_rollback_points");
    println!("0. 返回");
}

fn print_shop_menu() {
    println!();
    println!("--- 商店 / 游戏 API ---");
    println!("1. games_list (官方)");
    println!("2. games_list (第三方)");
    println!("3. games_kind_list");
    println!("4. games_version_list");
    println!("5. shop_list");
    println!("6. games_custom_list");
    println!("7. games_custom_version_list");
    println!("0. 返回");
}

fn print_dev_menu() {
    println!();
    println!("--- 开发者 API ---");
    println!("1. image_list");
    println!("2. image_detail");
    println!("3. image_versions");
    println!("4. image_feedback");
    println!("0. 返回");
}

async fn auth_menu(client: &SimpfunClient) -> Result<(), SdkError> {
    loop {
        print_auth_menu();
        let choice = read_line("选择操作: ");

        match choice.as_str() {
            "1" => {
                let info = client.user().auth_info().await?;
                println!("{:#?}", info);
            }
            "2" => {
                let data = client.user().announcement_list().await?;
                println!("公告数: {}", data.list.len());
                for item in data.list.iter().take(10) {
                    println!("  [{}] {}", item.id, item.title);
                }
            }
            "3" => {
                let data = client.user().point_history().await?;
                println!("积分记录数: {}", data.list.len());
                for item in data.list.iter().take(10) {
                    println!("  [{}] {} | {}", item.id, item.point, item.comment);
                }
            }
            "4" => {
                let data = client.user().diamond_history().await?;
                println!("钻石记录数: {}", data.list.len());
                for item in data.list.iter().take(10) {
                    println!("  [{}] {} | {}", item.id, item.diamond, item.comment);
                }
            }
            "5" => {
                let data = client.user().invite_info().await?;
                println!("{:#?}", data);
            }
            "6" => {
                client.user().logout().await?;
                println!("已登出");
            }
            "0" => break,
            _ => println!("无效选项"),
        }
    }

    Ok(())
}

async fn instance_menu(client: &SimpfunClient) -> Result<(), SdkError> {
    loop {
        print_instance_menu();
        let choice = read_line("选择操作: ");

        match choice.as_str() {
            "1" => {
                let data = client.user().ins_list().await?;
                println!("实例数: {}", data.list.len());
                for item in &data.list {
                    println!("  [{}] {:?} | state={}", item.id, item.name, item.state);
                }
            }
            "2" => {
                if let Some(id) = read_i64("请输入实例 ID: ") {
                    let data = client.user().ins_detail(id).await?;
                    println!("{:#?}", data);
                } else {
                    println!("实例 ID 无效");
                }
            }
            "3" => {
                if let Some(id) = read_i64("请输入实例 ID: ") {
                    let data = client.user().ins_ws_init(id).await?;
                    println!("{:#?}", data);
                } else {
                    println!("实例 ID 无效");
                }
            }
            "4" => {
                if let Some(id) = read_i64("请输入实例 ID: ") {
                    let data = client.user().ins_sftp(id).await?;
                    println!("{:#?}", data);
                } else {
                    println!("实例 ID 无效");
                }
            }
            "5" => {
                if let Some(id) = read_i64("请输入实例 ID: ") {
                    let data = client.user().ins_tasks(id).await?;
                    println!("{:#?}", data);
                } else {
                    println!("实例 ID 无效");
                }
            }
            "6" => {
                if let Some(id) = read_i64("请输入实例 ID: ") {
                    let data = client.user().ins_stat(id).await?;
                    println!("统计数量: {}", data.list.len());
                    for item in data.list.iter().take(10) {
                        println!("{:?}", item);
                    }
                } else {
                    println!("实例 ID 无效");
                }
            }
            "7" => {
                if let Some(id) = read_i64("请输入实例 ID: ") {
                    let data = client.user().ins_diamond_plan(id).await?;
                    println!("{:#?}", data);
                } else {
                    println!("实例 ID 无效");
                }
            }
            "8" => {
                if let Some(id) = read_i64("请输入实例 ID: ") {
                    let data = client.user().ins_support_info(id).await?;
                    println!("{:#?}", data);
                } else {
                    println!("实例 ID 无效");
                }
            }
            "0" => break,
            _ => println!("无效选项"),
        }
    }

    Ok(())
}

async fn file_menu(client: &SimpfunClient) -> Result<(), SdkError> {
    loop {
        print_file_menu();
        let choice = read_line("选择操作: ");

        match choice.as_str() {
            "1" => {
                if let Some(id) = read_i64("请输入实例 ID: ") {
                    let path = read_line("请输入路径(如 / 或 /home/container): ");
                    let data = client.user().ins_file_list(id, &path).await?;
                    println!("文件数: {}", data.list.len());
                    for item in data.list.iter().take(20) {
                        println!("  {:?} | file={} | size={:?}", item.name, item.file, item.size);
                    }
                } else {
                    println!("实例 ID 无效");
                }
            }
            "2" => {
                if let Some(id) = read_i64("请输入实例 ID: ") {
                    let path = read_line("请输入文件路径: ");
                    let data = client.user().ins_file_fetch(id, &path).await?;
                    println!("内容长度: {}", data.content.len());
                    println!("{}", data.content);