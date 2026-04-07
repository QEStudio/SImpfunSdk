//! WebSocket 自动重连示例
//!
//! 展示 WebSocket 连接的健壮性：
//! - 自动重连机制
//! - Token 过期自动刷新
//! - 认证失败自动重新登录
//!
//! 说明：
//! - 登录属于 user() API
//! - WebSocket 连接入口仍然是 connect_ins_ws(&client, id)

use simpfun::{connect_ins_ws, SdkError, SimpfunClient, WsEvent};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();

    // 从环境变量获取配置
    let username = std::env::var("SIMPFUN_USERNAME")
        .or_else(|_| std::env::var("SIMPFUN_USER"))
        .expect("请设置 SIMPFUN_USERNAME 环境变量");
    let password = std::env::var("SIMPFUN_PASSWORD")
        .or_else(|_| std::env::var("SIMPFUN_PASS"))
        .expect("请设置 SIMPFUN_PASSWORD 环境变量");
    let instance_id: i64 = std::env::var("SIMPFUN_INSTANCE_ID")
        .expect("请设置 SIMPFUN_INSTANCE_ID 环境变量")
        .parse()?;

    println!("=== WebSocket 自动重连示例 ===\n");
    println!("实例 ID: {}", instance_id);

    let client = SimpfunClient::new()?;

    // 1. 初始登录
    println!("正在登录...");
    client
        .user()
        .login_and_set_token(&username, &password)
        .await?;
    println!("登录成功！\n");

    // 2. 重连循环
    loop {
        println!("连接实例 {} 的 WebSocket...", instance_id);

        match connect_ins_ws(&client, instance_id).await {
            Ok((rx, control, _stop)) => {
                println!("WebSocket 连接成功！\n");

                // 请求日志流
                control.send_logs().await;

                // 3. 事件处理循环
                while let Ok(event) = rx.recv_async().await {
                    match event {
                        WsEvent::AuthSuccess => println!("[WS] 认证成功"),
                        WsEvent::ConsoleOutput(s) => print!("{}", s),
                        WsEvent::InstallOutput(s) => print!("[安装] {}", s),
                        WsEvent::Status(s) => println!("[状态] {}", s),
                        WsEvent::Stats(s) => println!("[统计] {}", s),

                        WsEvent::TokenExpiring => {
                            println!("[Token] 即将过期，正在自动刷新...")
                        }
                        WsEvent::TokenRefreshing => println!("[Token] 正在刷新..."),
                        WsEvent::TokenRefreshed => println!("[Token] 刷新成功"),
                        WsEvent::TokenRefreshFailed(e) => {
                            eprintln!("[Token] 刷新失败: {}", e);
                            break;
                        }

                        WsEvent::Reconnecting(msg) => println!("[重连] {}", msg),

                        WsEvent::Error(e) => {
                            eprintln!("[错误] {}", e);
                            break;
                        }
                    }
                }

                println!("\nWebSocket 连接断开");
            }

            Err(e) => {
                eprintln!("连接失败: {}", e);

                // 4. 如果是认证失败，则尝试重新登录
                if let SdkError::AuthFailed = e {
                    println!("检测到认证失效，正在重新登录...");

                    let mut retry_interval = 1;
                    loop {
                        match client.user().login_and_set_token(&username, &password).await {
                            Ok(_) => {
                                println!("重新登录成功！");
                                break;
                            }
                            Err(login_err) => {
                                eprintln!(
                                    "重新登录失败: {}, {}秒后重试...",
                                    login_err, retry_interval
                                );
                                sleep(Duration::from_secs(retry_interval)).await;
                                retry_interval = (retry_interval * 2).min(60);
                            }
                        }
                    }
                }
            }
        }

        // 5. 等待后重连
        println!("5秒后尝试重连...\n");
        sleep(Duration::from_secs(5)).await;
    }
}