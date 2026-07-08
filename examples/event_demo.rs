//! 事件轮询系统示例
//!
//! 展示如何使用 EventManager 监控数据变更：
//! - 用户信息变更
//! - 实例列表变更
//! - 实例详情变更
//! - 公告变更
//! - 离线/错误检测
//!
//! 说明：
//! - 事件轮询内部会调用 user() 相关 API
//! - 你只需要正常创建 SimpfunClient 并登录即可

use simpfun::{Event, EventManager, SimpfunClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simpfun SDK 事件轮询示例 ===\n");

    // 1. 初始化客户端
    let client = SimpfunClient::new()?;

    // 2. 登录（优先使用 Token，其次使用用户名密码）
    let token_env = std::env::var("SIMPFUN_TOKEN").ok();
    let user_env = std::env::var("SIMPFUN_USERNAME").ok();
    let pass_env = std::env::var("SIMPFUN_PASSWORD").ok();

    if let Some(t) = token_env {
        client.set_token(t);
        println!("使用 Token 登录");
    } else if let (Some(u), Some(p)) = (user_env, pass_env) {
        client.user().login_and_set_token(&u, &p).await?;
        println!("用户名密码登录成功");
    } else {
        eprintln!("请设置环境变量:");
        eprintln!("  SIMPFUN_TOKEN 或 SIMPFUN_USERNAME + SIMPFUN_PASSWORD");
        eprintln!("\n示例:");
        eprintln!("  export SIMPFUN_USERNAME=\"xxx\"");
        eprintln!("  export SIMPFUN_PASSWORD=\"yyy\"");
        eprintln!("  cargo run --example event_demo");
        return Ok(());
    }

    // 3. 获取实例 ID 列表，用于监控详情变化
    let ids: Vec<i64> = match client.user().ins_list().await {
        Ok(list) => {
            println!("发现 {} 个实例", list.list.len());
            list.list.iter().map(|i| i.id).collect()
        }
        Err(e) => {
            eprintln!("获取实例列表失败，将只监听公共主题: {}", e);
            Vec::new()
        }
    };

    // 4. 创建并启动事件管理器
    let manager = EventManager::new(client.clone()).watch_ins_details(ids);

    println!("启动事件监听...\n");

    let (rx, stop) = manager.start(None).await?;

    // 5. 事件处理循环
    println!("监听中（按 Ctrl+C 退出）...\n");

    while let Ok(event) = rx.recv_async().await {
        match event {
            Event::Heartbeat => {
                println!("[心跳] SDK 运行正常");
            }

            Event::AuthInfo(info) => {
                println!(
                    "[用户] 用户名: {} | 积分: {} | 钻石: {}",
                    info.info.username, info.info.point, info.info.diamond
                );
            }

            Event::InsList(list) => {
                println!("[实例] 数量: {}", list.list.len());
            }

            Event::InsDetail(detail) => {
                println!(
                    "[详情] 实例[{}] 状态: {} | CPU: {} | 内存: {}",
                    detail.data.id, detail.data.status, detail.data.cpu, detail.data.ram
                );
            }

            Event::Announcement(list) => {
                println!("[公告] 当前公告数量: {}", list.list.len());
                if let Some(first) = list.list.first() {
                    println!("  最新公告: {}", first.title);
                }
            }

            Event::Point(point) => {
                println!("[积分] 记录数: {}", point.list.len());
                if let Some(first) = point.list.first() {
                    println!("  最近积分变化: {} | {}", first.point, first.comment);
                }
            }

            Event::Diamond(diamond) => {
                println!("[钻石] 记录数: {}", diamond.list.len());
                if let Some(first) = diamond.list.first() {
                    println!("  最近钻石变化: {} | {}", first.diamond, first.comment);
                }
            }

            Event::Invite(invite) => {
                println!(
                    "[邀请] 邀请码: {} | 注册次数: {}",
                    invite.data.invite_code, invite.data.register_times
                );
            }

            Event::Offline(reason) => {
                println!("[离线] 原因: {}", reason);
                if reason.contains("AuthInfo") && reason.contains("400") {
                    println!("[警告] 账户可能已失效！");
                }
            }

            Event::Error(err) => {
                eprintln!("[错误] {}", err);
            }
        }
    }

    stop.stop().await;
    println!("\n事件监听已停止");
    Ok(())
}
