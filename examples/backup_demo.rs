//! 备份与回滚 API 示例
//! 
//! 演示以下 API：
//! - ins_backup_list: 备份列表
//! - ins_backup_create: 创建备份
//! - ins_backup_rename: 重命名备份
//! - ins_backup_restore: 恢复备份
//! - ins_backup_delete: 删除备份
//! - ins_backup_download: 获取下载链接
//! - ins_rollback_points: 回滚时间点列表
//! - ins_rollback_create: 执行回滚

use simpfun_sdk::{SimpfunClient, SdkError};

fn get_credentials() -> (String, String, i64) {
    let username = std::env::var("SIMPFUN_USERNAME")
        .expect("请设置 SIMPFUN_USERNAME 环境变量");
    let password = std::env::var("SIMPFUN_PASSWORD")
        .expect("请设置 SIMPFUN_PASSWORD 环境变量");
    let instance_id: i64 = std::env::var("SIMPFUN_INSTANCE_ID")
        .expect("请设置 SIMPFUN_INSTANCE_ID 环境变量")
        .parse()
        .expect("SIMPFUN_INSTANCE_ID 必须是数字");
    (username, password, instance_id)
}

#[tokio::main]
async fn main() -> Result<(), SdkError> {
    println!("=== 备份与回滚 API 示例 ===\n");

    let (username, password, instance_id) = get_credentials();
    let client = SimpfunClient::new()?;
    client.login_and_set_token(&username, &password).await?;
    println!("登录成功，实例 ID: {}\n", instance_id);

    // 1. 备份列表
    println!("--- ins_backup_list ---");
    let backups = client.ins_backup_list(instance_id).await?;
    println!("备份数量: {}", backups.list.len());
    
    let first_backup_id = backups.list.first().map(|b| b.id);
    for backup in &backups.list {
        println!(
            "  [{}] {} | 大小: {} MB | 有效期: {}",
            backup.id,
            backup.tag,
            backup.size / 1024 / 1024,
            backup.valid_time
        );
    }

    // 2. 回滚时间点列表
    println!("\n--- ins_rollback_points ---");
    match client.ins_rollback_points(instance_id).await {
        Ok(points) => {
            println!("可用回滚时间点: {}", points.list.len());
            for point in points.list.iter().take(5) {
                println!("  {}", point);
            }
        }
        Err(e) => println!("获取回滚时间点失败: {}", e),
    }

    // 3. 获取备份下载链接
    if let Some(backup_id) = first_backup_id {
        println!("\n--- ins_backup_download ---");
        match client.ins_backup_download(instance_id, backup_id).await {
            Ok(download) => {
                println!("下载 UUID: {}", download.uuid);
            }
            Err(e) => println!("获取下载链接失败: {}", e),
        }
    }

    // 以下操作会修改备份，默认注释掉
    println!("\n--- 以下为修改操作（已注释） ---");

    // 创建备份
    // println!("\n--- ins_backup_create ---");
    // let create = client.ins_backup_create(instance_id, "手动备份").await?;
    // println!("创建结果: {:?}", create.msg);

    // 重命名备份
    // if let Some(backup_id) = first_backup_id {
    //     println!("\n--- ins_backup_rename ---");
    //     let rename = client.ins_backup_rename(instance_id, backup_id, "新名称").await?;
    //     println!("重命名结果: {:?}", rename.msg);
    // }

    // 恢复备份
    // if let Some(backup_id) = first_backup_id {
    //     println!("\n--- ins_backup_restore ---");
    //     let restore = client.ins_backup_restore(instance_id, backup_id).await?;
    //     println!("恢复结果: {:?}", restore.msg);
    // }

    // 删除备份
    // if let Some(backup_id) = first_backup_id {
    //     println!("\n--- ins_backup_delete ---");
    //     let delete = client.ins_backup_delete(instance_id, backup_id).await?;
    //     println!("删除结果: {:?}", delete.msg);
    // }

    // 执行回滚
    // println!("\n--- ins_rollback_create ---");
    // let rollback = client.ins_rollback_create(instance_id, "2024-01-15T10:00:00").await?;
    // println!("回滚结果: {:?}", rollback.msg);

    println!("\n=== 示例结束 ===");
    Ok(())
}
