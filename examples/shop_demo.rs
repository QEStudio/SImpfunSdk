//! 商店与游戏镜像 API 示例
//! 
//! 演示以下 API：
//! - shop_list: 商店套餐列表
//! - games_list: 游戏列表
//! - games_kind_list: 游戏种类列表
//! - games_version_list: 游戏版本列表
//! - games_custom_list: 第三方镜像列表
//! - games_custom_version_list: 第三方版本列表

use simpfun_sdk::{SimpfunClient, SdkError};

fn get_credentials() -> (String, String) {
    let username = std::env::var("SIMPFUN_USERNAME")
        .expect("请设置 SIMPFUN_USERNAME 环境变量");
    let password = std::env::var("SIMPFUN_PASSWORD")
        .expect("请设置 SIMPFUN_PASSWORD 环境变量");
    (username, password)
}

#[tokio::main]
async fn main() -> Result<(), SdkError> {
    println!("=== 商店与游戏镜像 API 示例 ===\n");

    let (username, password) = get_credentials();
    let client = SimpfunClient::new()?;
    client.login_and_set_token(&username, &password).await?;
    println!("登录成功\n");

    // 1. 游戏列表（官方镜像）
    println!("--- games_list (官方镜像) ---");
    let games = client.games_list(false).await?;
    println!("游戏数量: {}", games.list.len());
    
    let first_game_id = games.list.first().map(|g| g.id);
    for game in games.list.iter().take(10) {
        println!(
            "  [{}] {}",
            game.id, game.name
        );
    }

    // 2. 游戏列表（第三方镜像）
    println!("\n--- games_list (第三方镜像) ---");
    let custom_games = client.games_list(true).await?;
    println!("第三方镜像数量: {}", custom_games.list.len());
    for game in custom_games.list.iter().take(5) {
        println!("  [{}] {}", game.id, game.name);
    }

    // 3. 游戏种类列表
    if let Some(game_id) = first_game_id {
        println!("\n--- games_kind_list (game_id={}) ---", game_id);
        let kinds = client.games_kind_list(game_id, 0).await?;
        println!("种类数量: {}", kinds.list.len());
        
        let first_kind_id = kinds.list.first().map(|k| k.id);
        for kind in kinds.list.iter().take(10) {
            println!(
                "  [{}] {} | 描述: {}",
                kind.id, kind.name, kind.description
            );
        }

        // 4. 游戏版本列表
        if let Some(kind_id) = first_kind_id {
            println!("\n--- games_version_list (kind_id={}) ---", kind_id);
            let versions = client.games_version_list(kind_id).await?;
            println!("版本数量: {}", versions.list.len());
            println!("Windows: {}", versions.is_windows);
            
            let first_version_id = versions.list.first().map(|v| v.id);
            for version in versions.list.iter().take(10) {
                println!(
                    "  [{}] {} | 描述: {}",
                    version.id, version.name, version.description
                );
            }

            // 5. 商店套餐列表
            if let Some(version_id) = first_version_id {
                println!("\n--- shop_list (version_id={}) ---", version_id);
                let shop = client.shop_list(version_id).await?;
                println!("套餐数量: {}", shop.list.len());
                for item in shop.list.iter().take(10) {
                    println!(
                        "  [{}] {} | CPU: {} 核 | 内存: {} MB | 磁盘: {} MB | 价格: {} 积分/月",
                        item.id, item.spec, item.cpu, item.ram, item.disk, item.point
                    );
                }
            }
        }

        // 6. 第三方镜像列表
        println!("\n--- games_custom_list (game_id={}) ---", game_id);
        match client.games_custom_list(game_id, 0).await {
            Ok(custom) => {
                println!("第三方镜像数量: {}", custom.list.len());
                let first_custom_kind_id = custom.list.first().map(|k| k.id);
                for kind in custom.list.iter().take(5) {
                    println!("  [{}] {} | 赞: {} | 踩: {}", kind.id, kind.name, kind.like, kind.dislike);
                }

                // 7. 第三方版本列表
                if let Some(custom_kind_id) = first_custom_kind_id {
                    println!("\n--- games_custom_version_list (kind_id={}) ---", custom_kind_id);
                    match client.games_custom_version_list(custom_kind_id).await {
                        Ok(versions) => {
                            println!("版本数量: {}", versions.list.len());
                            println!("Windows: {}", versions.is_windows);
                            for version in versions.list.iter().take(5) {
                                println!(
                                    "  [{}] {} | 大小: {} | 赞: {}",
                                    version.id, version.name, version.size, version.like
                                );
                            }
                        }
                        Err(e) => println!("获取第三方版本失败: {}", e),
                    }
                }
            }
            Err(e) => println!("获取第三方镜像失败: {}", e),
        }
    }

    println!("\n=== 示例结束 ===");
    Ok(())
}
