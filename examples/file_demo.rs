//! 文件操作 API 示例
//!
//! 演示以下 API：
//! - user().ins_file_list: 列出文件/目录
//! - user().ins_file_fetch: 获取文件内容
//! - user().ins_file_save: 保存文件
//! - user().ins_file_create: 创建文件/目录
//! - user().ins_file_rename: 重命名
//! - user().ins_file_delete: 删除
//! - user().ins_file_archive: 压缩
//! - user().ins_file_unarchive: 解压
//! - user().ins_file_paste: 粘贴
//! - user().ins_file_copy: 复制

use simpfun::{SdkError, SimpfunClient};

fn get_credentials() -> (String, String, i64) {
    let username = std::env::var("SIMPFUN_USERNAME").expect("请设置 SIMPFUN_USERNAME 环境变量");
    let password = std::env::var("SIMPFUN_PASSWORD").expect("请设置 SIMPFUN_PASSWORD 环境变量");
    let instance_id: i64 = std::env::var("SIMPFUN_INSTANCE_ID")
        .expect("请设置 SIMPFUN_INSTANCE_ID 环境变量")
        .parse()
        .expect("SIMPFUN_INSTANCE_ID 必须是数字");
    (username, password, instance_id)
}

#[tokio::main]
async fn main() -> Result<(), SdkError> {
    println!("=== 文件操作 API 示例 ===\n");

    let (username, password, instance_id) = get_credentials();

    // 1. 创建客户端并登录
    let client = SimpfunClient::new()?;
    client
        .user()
        .login_and_set_token(&username, &password)
        .await?;
    println!("登录成功，实例 ID: {}\n", instance_id);

    // 2. 列出根目录文件
    println!("--- user().ins_file_list (根目录) ---");
    let root_files = client.user().ins_file_list(instance_id, "/").await?;
    println!("文件/目录数量: {}", root_files.list.len());
    for file in root_files.list.iter().take(10) {
        let type_str = if file.file { "文件" } else { "目录" };
        let size_str = file
            .size
            .map(|s| format!("{} B", s))
            .unwrap_or_else(|| "-".to_string());
        println!(
            "  [{}] {} | 大小: {} | 修改: {:?}",
            type_str, file.name, size_str, file.modified_at
        );
    }

    // 3. 列出特定目录
    println!("\n--- user().ins_file_list (/home/container) ---");
    match client
        .user()
        .ins_file_list(instance_id, "/home/container")
        .await
    {
        Ok(files) => {
            println!("文件/目录数量: {}", files.list.len());
            for file in files.list.iter().take(10) {
                println!(
                    "  {} ({})",
                    file.name,
                    if file.file { "文件" } else { "目录" }
                );
            }
        }
        Err(e) => println!("列出目录失败: {}", e),
    }

    // 4. 获取文件内容
    println!("\n--- user().ins_file_fetch ---");
    let config_files = vec![
        "/home/container/server.properties",
        "/home/container/config.yml",
        "/home/container/server-icon.png",
    ];

    for path in config_files {
        match client.user().ins_file_fetch(instance_id, path).await {
            Ok(content) => {
                let preview = if content.content.len() > 200 {
                    format!(
                        "{}... (共 {} 字符)",
                        &content.content[..200],
                        content.content.len()
                    )
                } else {
                    content.content.clone()
                };
                println!("  {}:\n    {}", path, preview.replace('\n', "\n    "));
            }
            Err(_) => {
                // 文件不存在或读取失败时跳过
            }
        }
    }

    // 以下操作会修改文件，默认注释掉
    println!("\n--- 以下为修改操作（已注释） ---");

    // 保存文件
    // println!("\n--- user().ins_file_save ---");
    // let save = client
    //     .user()
    //     .ins_file_save(instance_id, "/home/container/test.txt", "Hello World!")
    //     .await?;
    // println!("保存结果: {:?}", save.msg);

    // 创建文件
    // println!("\n--- user().ins_file_create (文件) ---");
    // let create_file = client
    //     .user()
    //     .ins_file_create(instance_id, "file", "/home/container", "new_file.txt")
    //     .await?;
    // println!("创建结果: {:?}", create_file.msg);

    // 创建目录
    // println!("\n--- user().ins_file_create (目录) ---");
    // let create_dir = client
    //     .user()
    //     .ins_file_create(instance_id, "folder", "/home/container", "new_folder")
    //     .await?;
    // println!("创建结果: {:?}", create_dir.msg);

    // 重命名文件
    // println!("\n--- user().ins_file_rename ---");
    // let rename = client
    //     .user()
    //     .ins_file_rename(
    //         instance_id,
    //         "/home/container/old.txt",
    //         "/home/container/new.txt",
    //     )
    //     .await?;
    // println!("重命名结果: {:?}", rename.msg);

    // 删除文件
    // println!("\n--- user().ins_file_delete ---");
    // let delete = client
    //     .user()
    //     .ins_file_delete(
    //         instance_id,
    //         vec![
    //             "/home/container/file1.txt".to_string(),
    //             "/home/container/file2.txt".to_string(),
    //         ],
    //     )
    //     .await?;
    // println!("删除结果: {:?}", delete.msg);

    // 压缩文件
    // println!("\n--- user().ins_file_archive ---");
    // let archive = client
    //     .user()
    //     .ins_file_archive(
    //         instance_id,
    //         "/home/container",
    //         vec!["file1.txt".to_string(), "file2.txt".to_string()],
    //         "zip", // 或 "tar.gz"
    //     )
    //     .await?;
    // println!("压缩结果: {:?}", archive.msg);

    // 解压文件
    // println!("\n--- user().ins_file_unarchive ---");
    // let unarchive = client
    //     .user()
    //     .ins_file_unarchive(instance_id, "/home/container", "archive.zip")
    //     .await?;
    // println!("解压结果: {:?}", unarchive.msg);

    // 粘贴文件
    // println!("\n--- user().ins_file_paste ---");
    // let paste = client
    //     .user()
    //     .ins_file_paste(
    //         instance_id,
    //         vec!["/home/container/file.txt".to_string()],
    //         "/home/container/backup/",
    //     )
    //     .await?;
    // println!("粘贴结果: {:?}", paste.msg);

    // 复制文件（复制到剪贴板）
    // println!("\n--- user().ins_file_copy ---");
    // let copy = client
    //     .user()
    //     .ins_file_copy(instance_id, "/home/container/file.txt")
    //     .await?;
    // println!("复制结果: {:?}", copy.msg);

    println!("\n=== 示例结束 ===");
    Ok(())
}
