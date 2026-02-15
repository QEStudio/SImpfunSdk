# Simpfun SDK

[![Crates.io](https://img.shields.io/crates/v/simpfun-sdk.svg)](https://crates.io/crates/simpfun-sdk)
[![Documentation](https://docs.rs/simpfun-sdk/badge.svg)](https://docs.rs/simpfun-sdk)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Simpfun API 的 Rust SDK，提供完整的 API 封装、WebSocket 实时通信和事件轮询系统。

## 功能特性

- **HTTP API 完整封装** - 登录、实例管理、文件操作、备份管理等
- **WebSocket 实时通信** - 控制台输出、状态变更、自动重连
- **事件轮询系统** - 极速探测 + 本地去重，高效监控数据变更
- **自动 Token 刷新** - WebSocket Token 过期自动续期
- **类型安全** - 完整的 Rust 类型定义

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
simpfun-sdk = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

### 可选特性

```toml
[dependencies]
simpfun-sdk = { version = "0.1", features = ["mimalloc"] }
```

- `mimalloc` - 使用 mimalloc 作为全局分配器（高性能场景）

## 快速开始

### 基础用法

```rust
use simpfun_sdk::{SimpfunClient, SdkError};

#[tokio::main]
async fn main() -> Result<(), SdkError> {
    // 创建客户端
    let client = SimpfunClient::new()?;
    
    // 登录并保存 Token
    client.login_and_set_token("username", "password").await?;
    
    // 获取用户信息
    let info = client.auth_info().await?;
    println!("用户: {}, 积分: {}, 钻石: {}", 
        info.info.username, info.info.point, info.info.diamond);
    
    // 获取实例列表
    let instances = client.ins_list().await?;
    for ins in &instances.list {
        println!("实例: id={}, name={:?}", ins.id, ins.name);
    }
    
    Ok(())
}
```

### WebSocket 实时通信

```rust
use simpfun_sdk::{SimpfunClient, connect_ins_ws, WsEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SimpfunClient::new()?;
    client.login_and_set_token("username", "password").await?;
    
    // 连接实例 WebSocket
    let (rx, control, stop) = connect_ins_ws(&client, 12345).await?;
    
    // 请求日志流
    control.send_logs().await;
    
    // 处理事件
    while let Ok(event) = rx.recv_async().await {
        match event {
            WsEvent::ConsoleOutput(msg) => print!("{}", msg),
            WsEvent::Status(status) => println!("状态: {}", status),
            WsEvent::AuthSuccess => println!("认证成功"),
            _ => {}
        }
    }
    
    stop.stop();
    Ok(())
}
```

### 事件轮询系统

```rust
use simpfun_sdk::{SimpfunClient, EventManager, Event, PollingConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SimpfunClient::new()?;
    client.login_and_set_token("username", "password").await?;
    
    // 获取实例 ID 列表用于监控详情
    let instances = client.ins_list().await?;
    let ids: Vec<i64> = instances.list.iter().map(|i| i.id).collect();
    
    // 创建事件管理器
    let manager = EventManager::new(client)
        .watch_ins_details(ids);
    
    // 启动事件监听
    let (rx, control, stop) = manager.start_with_control(None, None).await?;
    
    // 处理事件
    while let Ok(event) = rx.recv_async().await {
        match event {
            Event::Heartbeat => println!("心跳"),
            Event::AuthInfo(info) => println!("积分: {}", info.info.point),
            Event::InsList(list) => println!("实例数: {}", list.list.len()),
            Event::InsDetail(detail) => println!("实例状态: {}", detail.data.status),
            Event::Offline(reason) => println!("离线: {}", reason),
            Event::Error(msg) => eprintln!("错误: {}", msg),
            _ => {}
        }
    }
    
    stop.stop().await;
    Ok(())
}
```

## API 概览

### 认证相关

| 方法 | 说明 |
|------|------|
| `login` | 用户名密码登录 |
| `login_and_set_token` | 登录并自动保存 Token |
| `logout` | 登出 |
| `auth_info` | 获取用户信息 |
| `set_token` | 手动设置 Token |
| `clear_token` | 清除 Token |

### 实例管理

| 方法 | 说明 |
|------|------|
| `ins_list` | 获取实例列表 |
| `ins_detail` | 获取实例详情 |
| `ins_power` | 电源控制 (start/stop/kill) |
| `ins_rename` | 重命名实例 |
| `ins_reinstall` | 重装实例 |
| `ins_ws_init` | 获取 WebSocket 连接信息 |

### 文件操作

| 方法 | 说明 |
|------|------|
| `ins_file_list` | 列出文件 |
| `ins_file_fetch` | 获取文件内容 |
| `ins_file_save` | 保存文件 |
| `ins_file_create` | 创建文件/目录 |
| `ins_file_rename` | 重命名 |
| `ins_file_delete` | 删除 |
| `ins_file_archive` | 压缩 |
| `ins_file_unarchive` | 解压 |

### 备份管理

| 方法 | 说明 |
|------|------|
| `ins_backup_list` | 备份列表 |
| `ins_backup_create` | 创建备份 |
| `ins_backup_restore` | 恢复备份 |
| `ins_backup_delete` | 删除备份 |
| `ins_backup_download` | 获取下载链接 |
| `ins_rollback_points` | 回滚时间点 |
| `ins_rollback_create` | 执行回滚 |

### 其他

| 方法 | 说明 |
|------|------|
| `announcement_list` | 公告列表 |
| `point_history` | 积分历史 |
| `diamond_history` | 钻石历史 |
| `invite_info` | 邀请信息 |
| `shop_list` | 商店套餐 |
| `games_list` | 游戏列表 |

## 示例

查看 `examples/` 目录获取完整示例：

### 基础示例

| 示例 | 说明 | 环境变量 |
|------|------|----------|
| `auth_demo.rs` | 认证相关 API（登录、用户信息、积分历史等） | `SIMPFUN_USERNAME`, `SIMPFUN_PASSWORD` |
| `instance_demo.rs` | 实例管理 API（列表、详情、电源控制等） | `SIMPFUN_USERNAME`, `SIMPFUN_PASSWORD` |
| `file_demo.rs` | 文件操作 API（列表、读取、保存、压缩等） | `SIMPFUN_USERNAME`, `SIMPFUN_PASSWORD`, `SIMPFUN_INSTANCE_ID` |
| `backup_demo.rs` | 备份与回滚 API | `SIMPFUN_USERNAME`, `SIMPFUN_PASSWORD`, `SIMPFUN_INSTANCE_ID` |
| `shop_demo.rs` | 商店与游戏镜像 API | `SIMPFUN_USERNAME`, `SIMPFUN_PASSWORD` |

### 高级示例

| 示例 | 说明 |
|------|------|
| `event_demo.rs` | 事件轮询系统基础用法 |
| `ws_reconnect.rs` | WebSocket 自动重连与 Token 刷新 |
| `showcase.rs` | 完整功能演示（交互式） |

### 运行示例

```bash
# Linux/macOS
export SIMPFUN_USERNAME="your_username"
export SIMPFUN_PASSWORD="your_password"
export SIMPFUN_INSTANCE_ID="12345"  # 部分示例需要

cargo run --example auth_demo
cargo run --example instance_demo
cargo run --example file_demo
cargo run --example backup_demo
cargo run --example shop_demo

# Windows PowerShell
$env:SIMPFUN_USERNAME="your_username"
$env:SIMPFUN_PASSWORD="your_password"
cargo run --example auth_demo
```

## 错误处理

```rust
use simpfun_sdk::{SimpfunClient, SdkError};

async fn handle_errors(client: &SimpfunClient) {
    match client.auth_info().await {
        Ok(info) => println!("用户: {}", info.info.username),
        Err(SdkError::AuthFailed) => eprintln!("认证失败，Token 无效或已过期"),
        Err(SdkError::MissingToken) => eprintln!("未设置 Token"),
        Err(SdkError::Api { code, msg }) => eprintln!("API 错误: {} - {}", code, msg),
        Err(SdkError::Http(e)) => eprintln!("HTTP 错误: {}", e),
        Err(e) => eprintln!("其他错误: {}", e),
    }
}
```

## 配置选项

### 自定义基地址

```rust
let client = SimpfunClient::new()?
    .with_base_url("https://custom-api.example.com");
```

### 自定义轮询间隔

```rust
use simpfun_sdk::{EventIntervals, PollingConfig};

let intervals = EventIntervals {
    heartbeat_secs: 60,
    announcement_secs: 120,
    ..Default::default()
};

let polling = PollingConfig {
    fast_secs: 2,      // 极速探测间隔
    safety_secs: 120,  // 安全兜底间隔
};

let (rx, _, stop) = manager.start_with_control(Some(intervals), Some(polling)).await?;
```

## 许可证

MIT License
