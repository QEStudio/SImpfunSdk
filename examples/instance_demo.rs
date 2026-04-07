//! 实例管理 API 示例
//!
//! 演示以下 API：
//! - user().ins_list: 实例列表
//! - user().ins_detail: 实例详情
//! - user().ins_power: 电源控制 (start/stop/kill)
//! - user().ins_rename: 重命名实例
//! - user().ins_change: 更换套餐
//! - user().ins_reinstall: 重装实例
//! - user().ins_ws_init: WebSocket 连接信息
//! - user().ins_sftp: SFTP 信息
//! - user().ins_tasks: 任务列表
//! - user().ins_stat: 历史统计
//! - user().ins_allocation_default: 设置默认端口
//! - user().ins_support_*: 技术支持相关
//! - user().ins_diamond_plan: 钻石套餐

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
    println!("=== 实例管理 API 示例 ===\n");

    let (username, password) = get_credentials();

    // 1. 创建客户端并登录
    let client = SimpfunClient::new()?;
    client
        .user()
        .login_and_set_token(&username, &password)
        .await?;
    println!("登录成功\n");

    // 2. 实例列表
    println!("--- user().ins_list ---");
    let instances = client.user().ins_list().await?;
    println!("实例数量: {}", instances.list.len());

    let first_id = instances.list.first().map(|i| i.id);
    for ins in &instances.list {
        println!(
            "  [{}] {:?} | 状态: {} | CPU: {} | 内存: {} | 磁盘: {}",
            ins.id, ins.name, ins.state, ins.cpu, ins.ram, ins.disk
        );
    }

    let instance_id = match first_id {
        Some(id) => id,
        None => {
            println!("\n没有实例，跳过后续演示");
            return Ok(());
        }
    };

    // 3. 实例详情
    println!("\n--- user().ins_detail (id={}) ---", instance_id);
    let detail = client.user().ins_detail(instance_id).await?;
    println!("名称: {:?}", detail.data.name);
    println!("状态: {}", detail.data.status);
    println!("UUID: {}", detail.data.uuid);
    println!("CPU: {} 核", detail.data.cpu);
    println!("内存: {} MB", detail.data.ram);
    println!("磁盘: {} MB", detail.data.disk);
    println!(
        "利用率: CPU {:.1}% | 内存 {} MB | 磁盘 {} MB",
        detail.data.utilization.cpu_absolute,
        detail.data.utilization.memory_bytes / 1024 / 1024,
        detail.data.utilization.disk_bytes / 1024 / 1024
    );
    println!(
        "游戏: {} / {} / {}",
        detail.data.game_info.game_name,
        detail.data.game_info.kind_name,
        detail.data.game_info.version_name
    );
    println!(
        "入口: {}:{}",
        detail.data.default_allocation.ip,
        detail.data.default_allocation.port
    );

    // 4. WebSocket 连接信息
    println!("\n--- user().ins_ws_init ---");
    let ws_info = client.user().ins_ws_init(instance_id).await?;
    println!(
        "Token: {}...",
        &ws_info.data.token[..20.min(ws_info.data.token.len())]
    );
    println!("Socket: {}", ws_info.data.socket);

    // 5. SFTP 信息
    println!("\n--- user().ins_sftp ---");
    let sftp = client.user().ins_sftp(instance_id).await?;
    println!("主机: {}", sftp.data.ip);
    println!("端口: {}", sftp.data.port);
    println!("用户名: {}", sftp.data.user_name);
    println!(
        "密码: {}...",
        &sftp.data.password[..10.min(sftp.data.password.len())]
    );

    // 6. 任务列表
    println!("\n--- user().ins_tasks ---");
    let tasks = client.user().ins_tasks(instance_id).await?;
    println!("任务数量: {}", tasks.list.len());
    println!("运行中: {} | 等待中: {}", tasks.running, tasks.waiting);
    for task in tasks.list.iter().take(3) {
        println!(
            "  [{}] 状态: {} | {}",
            task.id, task.status, task.comment
        );
    }

    // 7. 历史统计
    println!("\n--- user().ins_stat ---");
    let stats = client.user().ins_stat(instance_id).await?;
    println!("统计记录数: {}", stats.list.len());
    for stat in stats.list.iter().take(3) {
        println!(
            "  时间戳: {} | CPU: {:.1}% | 内存: {} MB",
            stat.create_time_timestamp,
            stat.cpu_percent,
            stat.mem_used_bytes / 1024 / 1024
        );
    }

    // 8. 钻石套餐
    println!("\n--- user().ins_diamond_plan ---");
    match client.user().ins_diamond_plan(instance_id).await {
        Ok(plan) => {
            println!("套餐数量: {}", plan.list.len());
            println!("当前钻石: {}", plan.diamond);
            for p in plan.list.iter().take(3) {
                println!(
                    "  [{}] {} | CPU: {} | 内存: {} | 钻石: {}",
                    p.id, p.spec, p.cpu, p.ram, p.diamond
                );
            }
        }
        Err(e) => println!("获取钻石套餐失败: {}", e),
    }

    // 9. 技术支持信息
    println!("\n--- user().ins_support_info ---");
    match client.user().ins_support_info(instance_id).await {
        Ok(support) => {
            println!("开发者 UID: {}", support.data.dev_uid);
            println!("开发者 QQ: {}", support.data.dev_qq);
            println!("支持群: {}", support.data.support_group);
            println!("有效: {}", support.data.valid);
        }
        Err(e) => println!("获取技术支持信息失败: {}", e),
    }

    // 以下操作会修改实例状态，默认注释掉
    println!("\n--- 以下为修改操作（已注释） ---");

    // 电源控制
    // println!("\n--- user().ins_power ---");
    // 启动实例
    // let power = client.user().ins_power(instance_id, "start").await?;
    // println!("启动结果: {:?}", power.msg);
    // 停止实例
    // let power = client.user().ins_power(instance_id, "stop").await?;
    // println!("停止结果: {:?}", power.msg);
    // 强制杀死
    // let power = client.user().ins_power(instance_id, "kill").await?;
    // println!("强制停止结果: {:?}", power.msg);

    // 重命名
    // println!("\n--- user().ins_rename ---");
    // let rename = client.user().ins_rename(instance_id, "新名称").await?;
    // println!("重命名结果: {:?}", rename.msg);

    // 更换套餐
    // println!("\n--- user().ins_change ---");
    // let change = client.user().ins_change(instance_id, 12345).await?;
    // println!("更换结果: {:?}", change.msg);

    // 重装实例
    // println!("\n--- user().ins_reinstall ---");
    // let reinstall = client
    //     .user()
    //     .ins_reinstall(instance_id, 12345, false, true, false)
    //     .await?;
    // println!("重装结果: {:?}", reinstall.msg);

    // 设置默认端口
    // println!("\n--- user().ins_allocation_default ---");
    // let alloc = client
    //     .user()
    //     .ins_allocation_default(instance_id, 12345)
    //     .await?;
    // println!("设置结果: {:?}", alloc.msg);

    // 创建技术支持请求
    // println!("\n--- user().ins_support_create ---");
    // let support = client
    //     .user()
    //     .ins_support_create(instance_id, "问题描述")
    //     .await?;
    // println!("创建支持结果: {:?}", support.msg);

    // 结束技术支持
    // println!("\n--- user().ins_support_end ---");
    // let end = client
    //     .user()
    //     .ins_support_end(instance_id, "反馈内容")
    //     .await?;
    // println!("结束支持结果: {:?}", end.msg);

    println!("\n=== 示例结束 ===");
    Ok(())
}