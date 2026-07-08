//! 事件轮询系统
//!
//! 提供极速探测 + 本地去重的事件监控机制

use crate::models::{
    AnnouncementListResponse, AuthInfoResponse, DiamondHistoryResponse, InsDetailResponse,
    InsListResponse, InviteResponse, PointHistoryResponse,
};
use crate::{SdkError, SimpfunClient};
use futures_util::future::join_all;
use reqwest::StatusCode;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::{Duration, MissedTickBehavior, interval};

use crate::client::ResourceMeta;

#[derive(Debug, Clone)]
pub struct EventIntervals {
    pub heartbeat_secs: u64,
    pub announcement_secs: u64,
    pub point_secs: u64,
    pub diamond_secs: u64,
    pub ins_list_secs: u64,
    pub auth_info_secs: u64,
    pub invite_secs: u64,
}

impl Default for EventIntervals {
    fn default() -> Self {
        Self {
            heartbeat_secs: 30,
            announcement_secs: 60,
            point_secs: 120,
            diamond_secs: 120,
            ins_list_secs: 60,
            auth_info_secs: 60,
            invite_secs: 300,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Event {
    Heartbeat,
    Announcement(AnnouncementListResponse),
    Point(PointHistoryResponse),
    Diamond(DiamondHistoryResponse),
    InsList(InsListResponse),
    AuthInfo(AuthInfoResponse),
    Invite(InviteResponse),
    InsDetail(Box<InsDetailResponse>),
    Offline(String),
    Error(String),
}

impl Event {
    pub fn to_json(&self) -> Result<String, SdkError> {
        Ok(serde_json::to_string(self)?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Topic {
    Announcement,
    Point,
    Diamond,
    InsList,
    AuthInfo,
    Invite,
}

impl Topic {
    fn all() -> &'static [Topic] {
        use Topic::*;
        &[Announcement, Point, Diamond, InsList, AuthInfo, Invite]
    }

    fn critical() -> &'static [Topic] {
        use Topic::*;
        &[Announcement, Point, InsList]
    }
}

#[derive(Debug, Clone)]
enum CachedData {
    Announcement(AnnouncementListResponse),
    Point(PointHistoryResponse),
    Diamond(DiamondHistoryResponse),
    InsList(InsListResponse),
    AuthInfo(AuthInfoResponse),
    Invite(InviteResponse),
    InsDetail(Box<InsDetailResponse>),
}

#[derive(Debug, Default, Clone)]
struct MonitorState {
    last_len: Option<u64>,
    last_etag: Option<String>,
    last_data: Option<CachedData>,
}

#[derive(Debug, Clone)]
pub struct PollingConfig {
    pub fast_secs: u64,
    pub safety_secs: u64,
}

impl Default for PollingConfig {
    fn default() -> Self {
        Self {
            fast_secs: 1,
            safety_secs: 60,
        }
    }
}

#[derive(Clone)]
pub struct EventControl {
    tx: flume::Sender<EventCmd>,
}

impl EventControl {
    pub async fn force_refresh_all(&self) {
        let _ = self
            .tx
            .send_async(EventCmd::ForceRefresh { topics: None })
            .await;
    }

    pub async fn force_refresh_topics(&self, topics: Vec<Topic>) {
        let _ = self
            .tx
            .send_async(EventCmd::ForceRefresh {
                topics: Some(topics),
            })
            .await;
    }

    pub async fn set_watch_list(&self, ids: Vec<i64>) {
        let _ = self.tx.send_async(EventCmd::UpdateWatchList(ids)).await;
    }
}

#[derive(Debug)]
pub enum EventCmd {
    ForceRefresh { topics: Option<Vec<Topic>> },
    UpdateWatchList(Vec<i64>),
}

pub struct EventManager {
    client: SimpfunClient,
    watch_details: Vec<i64>,
}

impl EventManager {
    pub fn new(client: SimpfunClient) -> Self {
        Self {
            client,
            watch_details: Vec::new(),
        }
    }

    pub fn watch_ins_details(mut self, ids: Vec<i64>) -> Self {
        self.watch_details = ids;
        self
    }

    pub async fn start_with_control(
        self,
        intervals: Option<EventIntervals>,
        polling: Option<PollingConfig>,
    ) -> Result<(flume::Receiver<Event>, EventControl, EventStop), SdkError> {
        if self.client.token().is_none() {
            return Err(SdkError::MissingToken);
        }

        let cfg = intervals.unwrap_or_default();
        let poll = polling.unwrap_or_default();
        let (tx, rx) = flume::bounded(100);
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        let (cmd_tx, cmd_rx) = flume::bounded(16);

        let client = self.client;
        let tx_run = tx.clone();
        let mut states: HashMap<Topic, MonitorState> = HashMap::new();
        let mut states_detail: HashMap<i64, MonitorState> = HashMap::new();

        init_all_topics(&client, &tx_run, &mut states).await;
        init_all_details(&client, &tx_run, &mut states_detail, &self.watch_details).await;

        let mut heartbeat_int = interval(Duration::from_secs(cfg.heartbeat_secs));
        let mut fast_int = interval(Duration::from_secs(poll.fast_secs));
        let mut safety_int = interval(Duration::from_secs(poll.safety_secs));

        heartbeat_int.set_missed_tick_behavior(MissedTickBehavior::Skip);
        fast_int.set_missed_tick_behavior(MissedTickBehavior::Skip);
        safety_int.set_missed_tick_behavior(MissedTickBehavior::Skip);

        let mut watch_details = self.watch_details.clone();

        let join = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = heartbeat_int.tick() => {
                        let _ = tx_run.send_async(Event::Heartbeat).await;
                    }
                    _ = fast_int.tick() => {
                        run_fast_probe(&client, &tx_run, &mut states, &mut states_detail, &watch_details).await;
                    }
                    _ = safety_int.tick() => {
                        run_safety_probe(&client, &tx_run, &mut states, &mut states_detail, &watch_details).await;
                    }
                    Ok(cmd) = cmd_rx.recv_async() => {
                        handle_command(cmd, &client, &tx_run, &mut states, &mut watch_details, &mut states_detail).await;
                    }
                    _ = &mut shutdown_rx => {
                        break;
                    }
                }
            }
        });

        let stop = EventStop {
            tx: shutdown_tx,
            join,
        };
        let control = EventControl { tx: cmd_tx };
        Ok((rx, control, stop))
    }

    pub async fn start(
        self,
        intervals: Option<EventIntervals>,
    ) -> Result<(flume::Receiver<Event>, EventStop), SdkError> {
        let (rx, _control, stop) = self.start_with_control(intervals, None).await?;
        Ok((rx, stop))
    }
}

pub struct EventStop {
    tx: oneshot::Sender<()>,
    join: JoinHandle<()>,
}

impl EventStop {
    pub async fn stop(self) {
        let _ = self.tx.send(());
        let _ = self.join.await;
    }
}

// ===== 初始化函数 =====

async fn init_all_topics(
    client: &SimpfunClient,
    tx: &flume::Sender<Event>,
    states: &mut HashMap<Topic, MonitorState>,
) {
    for &topic in Topic::all().iter() {
        if let Err(e) = fetch_and_dispatch(client, tx, states, topic).await {
            send_error_event(tx, &format!("{:?}", topic), e, "GET").await;
        }
    }
}

async fn init_all_details(
    client: &SimpfunClient,
    tx: &flume::Sender<Event>,
    states: &mut HashMap<i64, MonitorState>,
    watch_details: &[i64],
) {
    for &id in watch_details {
        if let Err(e) = fetch_and_dispatch_detail(client, tx, states, id).await {
            let _ = tx
                .send_async(Event::Error(format!("GET ins:{} {}", id, e)))
                .await;
        }
    }
}

// ===== 探测函数 =====

async fn run_fast_probe(
    client: &SimpfunClient,
    tx: &flume::Sender<Event>,
    states: &mut HashMap<Topic, MonitorState>,
    states_detail: &mut HashMap<i64, MonitorState>,
    watch_details: &[i64],
) {
    probe_critical_topics(client, tx, states).await;
    probe_details(client, tx, states_detail, watch_details).await;
}

async fn run_safety_probe(
    client: &SimpfunClient,
    tx: &flume::Sender<Event>,
    states: &mut HashMap<Topic, MonitorState>,
    states_detail: &mut HashMap<i64, MonitorState>,
    watch_details: &[i64],
) {
    for &topic in Topic::all().iter() {
        if let Ok(meta) = head_meta_topic(client, topic).await {
            let st = states.entry(topic).or_default();
            st.last_len = meta.len;
            st.last_etag = meta.etag;
        }

        if let Err(e) = fetch_and_dispatch(client, tx, states, topic).await {
            send_error_event(tx, &format!("{:?}", topic), e, "GET").await;
        }
    }

    for &id in watch_details {
        if let Ok(meta) = head_meta_detail(client, id).await {
            let st = states_detail.entry(id).or_default();
            st.last_len = meta.len;
            st.last_etag = meta.etag;
        }

        if let Err(e) = fetch_and_dispatch_detail(client, tx, states_detail, id).await {
            send_error_event(tx, &format!("ins:{}", id), e, "GET").await;
        }
    }
}

async fn probe_critical_topics(
    client: &SimpfunClient,
    tx: &flume::Sender<Event>,
    states: &mut HashMap<Topic, MonitorState>,
) {
    let critical = Topic::critical();
    let mut futures = Vec::with_capacity(critical.len());
    for &topic in critical.iter() {
        let c = client.clone();
        futures.push(async move { (topic, head_meta_topic(&c, topic).await) });
    }

    for (topic, res) in join_all(futures).await {
        match res {
            Ok(meta) => {
                let st = states.entry(topic).or_default();
                let changed = (meta.len.is_some() && meta.len != st.last_len)
                    || (meta.etag.is_some() && meta.etag != st.last_etag);

                if changed {
                    st.last_len = meta.len;
                    st.last_etag = meta.etag;
                    if let Err(e) = fetch_and_dispatch(client, tx, states, topic).await {
                        send_error_event(tx, &format!("{:?}", topic), e, "GET").await;
                    }
                }
            }
            Err(e) => {
                send_error_event(tx, &format!("{:?}", topic), e, "HEAD").await;
            }
        }
    }
}

async fn probe_details(
    client: &SimpfunClient,
    tx: &flume::Sender<Event>,
    states_detail: &mut HashMap<i64, MonitorState>,
    watch_details: &[i64],
) {
    let mut futures = Vec::with_capacity(watch_details.len());
    for &id in watch_details {
        let c = client.clone();
        futures.push(async move { (id, head_meta_detail(&c, id).await) });
    }

    for (id, res) in join_all(futures).await {
        match res {
            Ok(meta) => {
                let st = states_detail.entry(id).or_default();
                let changed = (meta.len.is_some() && meta.len != st.last_len)
                    || (meta.etag.is_some() && meta.etag != st.last_etag);

                if changed {
                    st.last_len = meta.len;
                    st.last_etag = meta.etag;
                    if let Err(e) = fetch_and_dispatch_detail(client, tx, states_detail, id).await {
                        send_error_event(tx, &format!("ins:{}", id), e, "GET").await;
                    }
                }
            }
            Err(e) => {
                send_error_event(tx, &format!("ins:{}", id), e, "HEAD").await;
            }
        }
    }
}

// ===== 命令处理 =====

async fn handle_command(
    cmd: EventCmd,
    client: &SimpfunClient,
    tx: &flume::Sender<Event>,
    states: &mut HashMap<Topic, MonitorState>,
    watch_details: &mut Vec<i64>,
    states_detail: &mut HashMap<i64, MonitorState>,
) {
    match cmd {
        EventCmd::ForceRefresh { topics } => {
            let list: Vec<Topic> = topics.unwrap_or_else(|| Topic::all().to_vec());
            for topic in list {
                if let Err(e) = fetch_and_dispatch(client, tx, states, topic).await {
                    send_error_event(tx, &format!("{:?}", topic), e, "GET").await;
                }
            }
        }
        EventCmd::UpdateWatchList(new_ids) => {
            *watch_details = new_ids;
            states_detail.retain(|id, _| watch_details.contains(id));
        }
    }
}

// ===== 辅助函数 =====

async fn send_error_event(tx: &flume::Sender<Event>, ctx: &str, e: SdkError, op: &str) {
    match e {
        SdkError::Status { status, .. } if status == StatusCode::BAD_REQUEST => {
            let _ = tx.send_async(Event::Offline(format!("{} 400", ctx))).await;
        }
        _ => {
            let _ = tx
                .send_async(Event::Error(format!("{} {} {}", op, ctx, e)))
                .await;
        }
    }
}

async fn head_meta_topic(client: &SimpfunClient, topic: Topic) -> Result<ResourceMeta, SdkError> {
    let path = match topic {
        Topic::Announcement => "/api/announcement",
        Topic::Point => "/api/pointhistory",
        Topic::Diamond => "/api/diamondhistory",
        Topic::InsList => "/api/ins/list",
        Topic::AuthInfo => "/api/auth/info",
        Topic::Invite => "/api/invite",
    };
    client.head_meta(path).await
}

async fn head_meta_detail(client: &SimpfunClient, id: i64) -> Result<ResourceMeta, SdkError> {
    client.head_meta(&format!("/api/ins/{}/detail", id)).await
}

async fn fetch_and_dispatch(
    client: &SimpfunClient,
    tx: &flume::Sender<Event>,
    states: &mut HashMap<Topic, MonitorState>,
    topic: Topic,
) -> Result<(), SdkError> {
    let st = states.entry(topic).or_default();

    match topic {
        Topic::Announcement => {
            let v = client.user().announcement_list().await?;
            if is_announcement_changed(&v, &st.last_data) {
                st.last_data = Some(CachedData::Announcement(v.clone()));
                let _ = tx.send_async(Event::Announcement(v)).await;
            }
        }
        Topic::Point => {
            let v = client.user().point_history().await?;
            if is_point_history_changed(&v, &st.last_data) {
                st.last_data = Some(CachedData::Point(v.clone()));
                let _ = tx.send_async(Event::Point(v)).await;
            }
        }
        Topic::Diamond => {
            let v = client.user().diamond_history().await?;
            if is_diamond_history_changed(&v, &st.last_data) {
                st.last_data = Some(CachedData::Diamond(v.clone()));
                let _ = tx.send_async(Event::Diamond(v)).await;
            }
        }
        Topic::InsList => {
            let v = client.user().ins_list().await?;
            if is_ins_list_changed(&v, &st.last_data) {
                st.last_data = Some(CachedData::InsList(v.clone()));
                let _ = tx.send_async(Event::InsList(v)).await;
            }
        }
        Topic::AuthInfo => {
            let v = client.user().auth_info().await?;
            if is_auth_info_changed(&v, &st.last_data) {
                st.last_data = Some(CachedData::AuthInfo(v.clone()));
                let _ = tx.send_async(Event::AuthInfo(v)).await;
            }
        }
        Topic::Invite => {
            let v = client.user().invite_info().await?;
            if is_invite_changed(&v, &st.last_data) {
                st.last_data = Some(CachedData::Invite(v.clone()));
                let _ = tx.send_async(Event::Invite(v)).await;
            }
        }
    }
    Ok(())
}

async fn fetch_and_dispatch_detail(
    client: &SimpfunClient,
    tx: &flume::Sender<Event>,
    states: &mut HashMap<i64, MonitorState>,
    id: i64,
) -> Result<(), SdkError> {
    let v = client.user().ins_detail(id).await?;
    let st = states.entry(id).or_default();

    if is_ins_detail_changed(&v, &st.last_data) {
        st.last_data = Some(CachedData::InsDetail(Box::new(v.clone())));
        let _ = tx.send_async(Event::InsDetail(Box::new(v))).await;
    }
    Ok(())
}

// ===== 变更检测函数 =====

fn is_announcement_changed(new: &AnnouncementListResponse, last: &Option<CachedData>) -> bool {
    match last {
        Some(CachedData::Announcement(old)) => {
            if new.list.len() != old.list.len() {
                return true;
            }
            new.list.first().map(|n| n.id) != old.list.first().map(|o| o.id)
                || new.list.last().map(|n| n.id) != old.list.last().map(|o| o.id)
        }
        _ => true,
    }
}

fn is_point_history_changed(new: &PointHistoryResponse, last: &Option<CachedData>) -> bool {
    match last {
        Some(CachedData::Point(old)) => {
            if new.list.len() != old.list.len() {
                return true;
            }
            new.list.first().map(|n| n.id) != old.list.first().map(|o| o.id)
        }
        _ => true,
    }
}

fn is_diamond_history_changed(new: &DiamondHistoryResponse, last: &Option<CachedData>) -> bool {
    match last {
        Some(CachedData::Diamond(old)) => {
            if new.list.len() != old.list.len() {
                return true;
            }
            new.list.first().map(|n| n.id) != old.list.first().map(|o| o.id)
        }
        _ => true,
    }
}

fn is_ins_list_changed(new: &InsListResponse, last: &Option<CachedData>) -> bool {
    match last {
        Some(CachedData::InsList(old)) => {
            if new.list.len() != old.list.len() {
                return true;
            }
            new.list.iter().map(|i| (i.id, i.state)).collect::<Vec<_>>()
                != old.list.iter().map(|i| (i.id, i.state)).collect::<Vec<_>>()
        }
        _ => true,
    }
}

fn is_auth_info_changed(new: &AuthInfoResponse, last: &Option<CachedData>) -> bool {
    match last {
        Some(CachedData::AuthInfo(old)) => {
            new.info.id != old.info.id
                || new.info.point != old.info.point
                || new.info.diamond != old.info.diamond
        }
        _ => true,
    }
}

fn is_invite_changed(new: &InviteResponse, last: &Option<CachedData>) -> bool {
    match last {
        Some(CachedData::Invite(old)) => {
            new.data.invite_code != old.data.invite_code
                || new.data.register_times != old.data.register_times
        }
        _ => true,
    }
}

fn is_ins_detail_changed(new: &InsDetailResponse, last: &Option<CachedData>) -> bool {
    match last {
        Some(CachedData::InsDetail(old)) => {
            new.data.status != old.data.status
                || new.data.utilization.cpu_absolute != old.data.utilization.cpu_absolute
                || new.data.utilization.memory_bytes != old.data.utilization.memory_bytes
        }
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_intervals_default() {
        let intervals = EventIntervals::default();
        assert_eq!(intervals.heartbeat_secs, 30);
        assert_eq!(intervals.announcement_secs, 60);
    }

    #[test]
    fn test_polling_config_default() {
        let config = PollingConfig::default();
        assert_eq!(config.fast_secs, 1);
        assert_eq!(config.safety_secs, 60);
    }

    #[test]
    fn test_topic_all() {
        let all = Topic::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn test_topic_critical() {
        let critical = Topic::critical();
        assert_eq!(critical.len(), 3);
    }

    #[test]
    fn test_event_to_json() {
        let event = Event::Heartbeat;
        let json = event.to_json().unwrap();
        assert!(json.contains("Heartbeat"));
    }
}
