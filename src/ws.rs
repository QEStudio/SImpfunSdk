use crate::{SdkError, SimpfunClient};
use tokio::sync::broadcast;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};
use tracing::{debug, error, warn, info};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[derive(Debug, Serialize)]
pub enum WsEvent {
    AuthSuccess,
    TokenExpiring,
    TokenRefreshing,
    TokenRefreshed,
    TokenRefreshFailed(String),
    Status(String),
    Stats(String),
    ConsoleOutput(String),
    InstallOutput(String),
    Error(String),
    Reconnecting(String),
}

impl WsEvent {
    pub fn to_json(&self) -> Result<String, SdkError> {
        Ok(serde_json::to_string(self)?)
    }
}

pub struct WsControl {
    tx: flume::Sender<Message>,
}

impl WsControl {
    pub async fn send_logs(&self) {
        let payload = serde_json::json!({
            "event": "send logs",
            "args": []
        }).to_string();
        if let Err(e) = self.tx.send_async(Message::Text(payload)).await {
            error!("发送 send logs 失败: {}", e);
        }
    }

    pub async fn set_state(&self, state: &str) {
        let payload = serde_json::json!({
            "event": "set state",
            "args": [state]
        }).to_string();
        if let Err(e) = self.tx.send_async(Message::Text(payload)).await {
            error!("发送 set state 失败: {}", e);
        }
    }

    pub async fn send_command(&self, cmd: &str) {
        let payload = serde_json::json!({
            "event": "send command",
            "args": [cmd]
        }).to_string();
        if let Err(e) = self.tx.send_async(Message::Text(payload)).await {
            error!("发送 command 失败: {}", e);
        }
    }
}

pub struct WsStop {
    tx: broadcast::Sender<()>,
}

impl WsStop {
    pub fn stop(self) {
        let _ = self.tx.send(());
    }
}

pub async fn connect_ins_ws(
    client: &SimpfunClient,
    id: i64,
) -> Result<(flume::Receiver<WsEvent>, WsControl, WsStop), SdkError> {
    let (tx_msg, rx_msg) = flume::bounded::<Message>(32);
    let (tx_evt, rx_evt) = flume::bounded::<WsEvent>(128);
    let (shutdown_tx, _) = broadcast::channel(1);

    let client_clone = client.clone();
    let mut shutdown_rx = shutdown_tx.subscribe();
    let tx_msg_inner = tx_msg.clone();
    let refreshing = Arc::new(AtomicBool::new(false));

    tokio::spawn(async move {
        let tx_msg = tx_msg_inner;
        let mut backoff_secs = 1u64;
        let max_backoff = 60u64;

        loop {
            if shutdown_rx.try_recv().is_ok() {
                break;
            }

            let (token, socket_url) = match client_clone.user().ins_ws_init(id).await {
                Ok(init) => (init.data.token, init.data.socket),
                Err(e) => {
                    let _ = tx_evt.send_async(WsEvent::Error(format!("获取WS信息失败: {}", e))).await;
                    let _ = tx_evt
                        .send_async(WsEvent::Reconnecting(format!("等待 {}s 后重试...", backoff_secs)))
                        .await;
                    sleep(Duration::from_secs(backoff_secs)).await;
                    backoff_secs = (backoff_secs * 2).min(max_backoff);
                    continue;
                }
            };

            match connect_async(&socket_url).await {
                Ok((ws_stream, _)) => {
                    info!("WebSocket 连接成功: {}", socket_url);
                    backoff_secs = 1;

                    let (mut ws_write, mut ws_read) = ws_stream.split();

                    let auth_payload = serde_json::json!({
                        "event": "auth",
                        "args": [token]
                    }).to_string();

                    if let Err(e) = ws_write.send(Message::Text(auth_payload)).await {
                        error!("发送 Auth 失败: {}", e);
                        continue;
                    }

                    refreshing.store(false, Ordering::SeqCst);

                    loop {
                        tokio::select! {
                            _ = shutdown_rx.recv() => {
                                info!("收到停止信号，退出 WebSocket 循环");
                                let _ = ws_write.close().await;
                                return;
                            }
                            msg_opt = rx_msg.recv_async() => {
                                match msg_opt {
                                    Ok(msg) => {
                                        if let Err(e) = ws_write.send(msg).await {
                                            error!("WS 发送失败: {}", e);
                                            break;
                                        }
                                    }
                                    Err(_) => {
                                        info!("控制通道关闭，退出");
                                        return;
                                    }
                                }
                            }
                            ws_msg = ws_read.next() => {
                                match ws_msg {
                                    Some(Ok(msg)) => {
                                        match msg {
                                            Message::Text(t) => {
                                                process_ws_message(
                                                    &t,
                                                    &tx_evt,
                                                    &client_clone,
                                                    &tx_msg,
                                                    id,
                                                    &socket_url,
                                                    &refreshing,
                                                ).await;
                                            }
                                            Message::Binary(_) => {}
                                            Message::Ping(d) => {
                                                if let Err(e) = ws_write.send(Message::Pong(d)).await {
                                                    error!("回复 Pong 失败: {}", e);
                                                    break;
                                                }
                                            }
                                            Message::Pong(_) => {}
                                            Message::Frame(_) => {}
                                            Message::Close(_) => {
                                                info!("WS Server Closed");
                                                break;
                                            }
                                        }
                                    }
                                    Some(Err(e)) => {
                                        error!("WS 读取错误: {}", e);
                                        break;
                                    }
                                    None => {
                                        info!("WS Stream Ended");
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    let _ = tx_evt
                        .send_async(WsEvent::Reconnecting(format!("连接断开，{}s 后重试...", backoff_secs)))
                        .await;
                    sleep(Duration::from_secs(backoff_secs)).await;
                    backoff_secs = (backoff_secs * 2).min(max_backoff);
                }
                Err(e) => {
                    error!("WS 连接失败: {}", e);
                    let _ = tx_evt.send_async(WsEvent::Error(format!("连接失败: {}", e))).await;
                    let _ = tx_evt
                        .send_async(WsEvent::Reconnecting(format!("等待 {}s 后重试...", backoff_secs)))
                        .await;
                    sleep(Duration::from_secs(backoff_secs)).await;
                    backoff_secs = (backoff_secs * 2).min(max_backoff);
                }
            }
        }
    });

    let control = WsControl { tx: tx_msg };
    let stop = WsStop { tx: shutdown_tx };
    Ok((rx_evt, control, stop))
}

#[derive(Deserialize)]
struct WsPacket<'a> {
    #[serde(borrow)]
    event: Cow<'a, str>,
    #[serde(borrow)]
    args: Option<Vec<Cow<'a, str>>>,
}

async fn process_ws_message(
    text: &str,
    tx_evt: &flume::Sender<WsEvent>,
    client: &SimpfunClient,
    tx_msg: &flume::Sender<Message>,
    id: i64,
    current_socket_url: &str,
    refreshing: &Arc<AtomicBool>,
) {
    if let Ok(v) = serde_json::from_str::<WsPacket>(text) {
        let ev = v.event.as_ref();
        match ev {
            "auth success" => {
                let _ = tx_evt.send_async(WsEvent::AuthSuccess).await;
                let payload = serde_json::json!({
                    "event": "send logs",
                    "args": []
                }).to_string();
                if let Err(e) = tx_msg.send_async(Message::Text(payload)).await {
                    error!("发送 send logs 失败: {}", e);
                }
            }
            "status" => {
                if let Some(args) = v.args {
                    if let Some(s) = args.first() {
                        let _ = tx_evt.send_async(WsEvent::Status(s.to_string())).await;
                    }
                }
            }
            "stats" => {
                if let Some(args) = v.args {
                    if let Some(s) = args.first() {
                        let _ = tx_evt.send_async(WsEvent::Stats(s.to_string())).await;
                    }
                }
            }
            "console output" => {
                if let Some(args) = v.args {
                    if let Some(s) = args.first() {
                        let _ = tx_evt.send_async(WsEvent::ConsoleOutput(s.to_string())).await;
                    }
                }
            }
            "install output" => {
                if let Some(args) = v.args {
                    if let Some(s) = args.first() {
                        let _ = tx_evt.send_async(WsEvent::InstallOutput(s.to_string())).await;
                    }
                }
            }
            "token expiring" => {
                if refreshing
                    .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                    .is_err()
                {
                    debug!("Token 刷新任务已在进行中，跳过");
                    return;
                }

                let _ = tx_evt.send_async(WsEvent::TokenExpiring).await;
                let client_inner = client.clone();
                let tx_msg_inner = tx_msg.clone();
                let tx_evt_inner = tx_evt.clone();
                let current_url = current_socket_url.to_string();
                let refreshing_inner = Arc::clone(refreshing);

                tokio::spawn(async move {
                    let _ = tx_evt_inner.send_async(WsEvent::TokenRefreshing).await;
                    match client_inner.user().ins_ws_init(id).await {
                        Ok(new_init) => {
                            if new_init.data.socket != current_url {
                                warn!("Token刷新后 URL 变更，连接可能失效");
                            }
                            let payload = serde_json::json!({
                                "event": "auth",
                                "args": [new_init.data.token]
                            }).to_string();
                            if let Err(e) = tx_msg_inner.send_async(Message::Text(payload)).await {
                                error!("发送刷新Token失败: {}", e);
                            } else {
                                let _ = tx_evt_inner.send_async(WsEvent::TokenRefreshed).await;
                            }
                        }
                        Err(e) => {
                            let _ = tx_evt_inner
                                .send_async(WsEvent::TokenRefreshFailed(e.to_string()))
                                .await;
                        }
                    }
                    refreshing_inner.store(false, Ordering::SeqCst);
                });
            }
            _ => {
                debug!("未知事件: {}", ev);
            }
        }
    } else {
        warn!("解析WS文本失败: {}", text);
        let _ = tx_evt
            .send_async(WsEvent::Error("解析WS文本失败".to_string()))
            .await;
    }
}