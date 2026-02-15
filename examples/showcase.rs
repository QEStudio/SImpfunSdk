//! 完整功能演示示例
//! 
//! 展示 SDK 的主要功能：
//! - 用户认证
//! - 实例管理
//! - WebSocket 实时通信
//! - 事件轮询系统

use simpfun_sdk::{
    SimpfunClient,
    EventManager,
    Event,
    Topic,
    PollingConfig,
    EventIntervals,
    SdkError,
    connect_ins_ws,
    WsEvent,
};

use tokio::time::{sleep, Duration};

async fn dump_ins(client: &SimpfunClient, id: i64) -> Result<(), SdkError> {
    match client.ins_detail(id).await {
        Ok(detail) => {
            println!(
                "实例详情: name={:?} status={} cpu={} ram={} disk={}",
                detail.data.name, detail.data.status, detail.data.cpu, detail.data.ram, detail.data.disk
            );
            println!(
                "标识: id={} uuid={} pro={} suspended={}",
                detail.data.id, detail.data.uuid, detail.data.is_pro, detail.data.is_suspended
            );
            println!(
                "利用率: mem={}B cpu={:.1}% disk={}B",
                detail.data.utilization.memory_bytes,
                detail.data.utilization.cpu_absolute,
                detail.data.utilization.disk_bytes,
            );
            println!(
                "游戏: {} / {} / {}",
                detail.data.game_info.game_name,
                detail.data.game_info.kind_name,
                detail.data.game_info.version_name
            );
            println!(
                "入口: {}:{}",
                detail.data.default_allocation.ip, detail.data.default_allocation.port
            );
        }
        Err(e) => eprintln!("获取实例详情失败: {}", e),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), SdkError> {
    println!("=== Simpfun SDK 功能演示 ===\n");

    // 读取凭据（推荐使用环境变量）
    let token_env = std::env::var("SIMPFUN_TOKEN").ok();
    let user_env = std::env::var("SIMPFUN_USERNAME").ok();
    let pass_env = std::env::var("SIMPFUN_PASSWORD").ok();

    // 1) 创建客户端并登录
    let client = SimpfunClient::new()?;
    let token_str = if let Some(t) = token_env {
        client.set_token(t.clone());
        println!("使用 Token 登录");
        t
    } else if let (Some(u), Some(p)) = (user_env, pass_env) {
        let token = client.login_and_set_token(&u, &p).await?;
        println!("用户名密码登录成功");
        token
    } else {
        eprintln!("请设置环境变量:");
        eprintln!("  SIMPFUN_TOKEN 或 SIMPFUN_USERNAME + SIMPFUN_PASSWORD");
        eprintln!("\n示例:");
        eprintln!("  export SIMPFUN_USERNAME=\"xxx\"");
        eprintln!("  export SIMPFUN_PASSWORD=\"yyy\"");
        eprintln!("  cargo run --example showcase");
        return Ok(());
    };

    // 2) 演示基础 API
    println!("\n--- 基础 API 演示 ---\n");
    
    match client.auth_info().await {
        Ok(v) => println!("用户: {} | 积分: {} | 钻石: {}", v.info.username, v.info.point, v.info.diamond),
        Err(e) => eprintln!("获取用户信息失败: {}", e),
    }

    let mut watch_ids: Vec<i64> = Vec::new();
    match client.ins_list().await {
        Ok(v) => {
            println!("实例数量: {}", v.list.len());
            watch_ids = v.list.iter().map(|ins| ins.id).collect();
            for ins in &v.list {
                println!("  - id={} name={:?} state={}", ins.id, ins.name, ins.state);
            }
        }
        Err(e) => eprintln!("获取实例列表失败: {}", e),
    }

    match client.announcement_list().await {
        Ok(v) => {
            println!("公告数量: {}", v.list.len());
            if let Some(first) = v.list.first() {
                println!("  最新: {}", first.title);
            }
        }
        Err(e) => eprintln!("获取公告失败: {}", e),
    }

    // 3) 演示实例详情
    if let Some(&first_id) = watch_ids.first() {
        println!("\n--- 实例详情演示 ---\n");
        dump_ins(&client, first_id).await?;
    }

    // 4) 演示 WebSocket（交互式）
    println!("\n--- WebSocket 演示 ---");
    if watch_ids.is_empty() {
        println!("没有实例，跳过 WebSocket 演示");
    } else {
        println!("可用实例: {:?}", watch_ids);
        println!("输入实例 ID 进行 WebSocket 连接（直接回车跳过）:");
        
        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf);
        
        if let Ok(id) = buf.trim().parse::<i64>() {
            println!("连接实例 {} 的 WebSocket...", id);
            
            match connect_ins_ws(&client, id).await {
                Ok((rx, control, stop)) => {
                    println!("WebSocket 连接成功！输入命令进行交互:");
                    println!("  logs     - 获取日志");
                    println!("  cmd <文本> - 发送命令");
                    println!("  exit     - 退出");
                    
                    control.send_logs().await;
                    
                    let ws_task = tokio::spawn(async move {
                        while let Ok(ev) = rx.recv_async().await {
                            match ev {
                                WsEvent::AuthSuccess => println!("[WS] 认证成功"),
                                WsEvent::ConsoleOutput(s) => print!("{}", s),
                                WsEvent::Status(s) => println!("[WS] 状态: {}", s),
                                WsEvent::Error(e) => eprintln!("[WS] 错误: {}", e),
                                _ => {}
                            }
                        }
                    });
                    
                    loop {
                        let mut input = String::new();
                        let _ = std::io::stdin().read_line(&mut input);
                        let cmd = input.trim();
                        
                        if cmd == "exit" {
                            break;
                        } else if cmd == "logs" {
                            control.send_logs().await;
                        } else if let Some(rest) = cmd.strip_prefix("cmd ") {
                            control.send_command(rest).await;
                        }
                    }
                    
                    stop.stop();
                    let _ = ws_task.await;
                }
                Err(e) => eprintln!("WebSocket 连接失败: {}", e),
            }
        }
    }

    // 5) 演示事件轮询系统
    println!("\n--- 事件轮询演示 ---\n");
    
    let polling = PollingConfig { 
        fast_secs: 1, 
        safety_secs: 10 
    };
    let intervals = EventIntervals { 
        heartbeat_secs: 5, 
        ..Default::default() 
    };

    let event_client = SimpfunClient::new()?;
    event_client.set_token(token_str);
    
    let manager = EventManager::new(event_client)
        .watch_ins_details(watch_ids);
    
    let (rx, control, stop) = manager
        .start_with_control(Some(intervals), Some(polling))
        .await?;

    let event_task = tokio::spawn(async move {
        while let Ok(ev) = rx.recv_async().await {
            match ev {
                Event::Heartbeat => println!("[Event] 心跳"),
                Event::AuthInfo(v) => println!("[Event] 用户: 积分={} 钻石={}", v.info.point, v.info.diamond),
                Event::InsList(v) => println!("[Event] 实例数: {}", v.list.len()),
                Event::InsDetail(v) => println!("[Event] 实例[{}] 状态={}", v.data.id, v.data.status),
                Event::Offline(msg) => println!("[Event] 离线: {}", msg),
                Event::Error(msg) => eprintln!("[Event] 错误: {}", msg),
                _ => {}
            }
        }
    });

    // 演示强制刷新
    sleep(Duration::from_secs(2)).await;
    println!("\n触发强制刷新: InsList + AuthInfo");
    control.force_refresh_topics(vec![Topic::InsList, Topic::AuthInfo]).await;

    sleep(Duration::from_secs(2)).await;
    println!("触发强制刷新: 全部主题");
    control.force_refresh_all().await;

    // 运行一段时间后停止
    println!("\n事件监听运行中，10秒后自动停止...");
    sleep(Duration::from_secs(10)).await;
    
    stop.stop().await;
    let _ = event_task.await;

    println!("\n=== 演示结束 ===");
    Ok(())
}
