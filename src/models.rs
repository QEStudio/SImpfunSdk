use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LoginResponse {
    pub code: i32,
    pub msg: String,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Announcement {
    #[serde(default)]
    pub show: bool,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub point: i64,
    pub diamond: i64,
    pub queue_time: i64,
    pub verified: bool,
    pub is_dev: bool,
    pub create_time: i64,
    pub qq: Option<i64>,
    pub pd: Option<String>,
    pub is_pro: bool,
    pub pro_valid: bool,
    #[serde(default)]
    pub announcement: Option<Announcement>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AuthInfoResponse {
    pub code: i32,
    pub info: UserInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Instance {
    pub id: i64,
    pub name: Option<String>,
    pub cpu: String,
    pub ram: String,
    pub disk: String,
    pub create_time: String,
    pub last_paid_time: String,
    pub area: i64,
    pub version_id: i64,
    pub state: i32,
    pub ptero_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InsListResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<Instance>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AnnouncementItem {
    pub id: i64,
    pub title: String,
    pub text: String,
    pub create_time: String,
    pub read_times: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AnnouncementListResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<AnnouncementItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PointHistoryItem {
    pub id: i64,
    pub point: i64,
    pub point_left: i64,
    pub create_time: String,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PointHistoryResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<PointHistoryItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiamondHistoryItem {
    pub id: i64,
    pub diamond: i64,
    pub diamond_left: i64,
    pub create_time: String,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiamondHistoryResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<DiamondHistoryItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteData {
    pub register_times: i64,
    pub register_verify_times: i64,
    pub register_total_income: i64,
    pub register_total_income_from_pro: i64,
    pub invite_code: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteResponse {
    pub code: i32,
    pub data: InviteData,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WsInitData {
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub socket: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WsInitResponse {
    pub code: i32,
    pub data: WsInitData,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PowerResponse {
    pub code: i32,
    #[serde(default)]
    pub status: bool,
    #[serde(default)]
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InsDetailResponse {
    pub code: i32,
    pub data: InsDetail,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct InsUtilization {
    #[serde(default)]
    pub memory_bytes: i64,
    #[serde(default)]
    pub cpu_absolute: f64,
    #[serde(default)]
    pub disk_bytes: i64,
    #[serde(default)]
    pub network_rx_bytes: i64,
    #[serde(default)]
    pub network_tx_bytes: i64,
    #[serde(default)]
    pub uptime: f64,
    #[serde(default)]
    pub disk_last_check_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct GameInfo {
    #[serde(default)]
    pub game_name: String,
    #[serde(default)]
    pub kind_name: String,
    #[serde(default)]
    pub version_name: String,
    #[serde(default)]
    pub game_id: i64,
    #[serde(default)]
    pub kind_id: i64,
    #[serde(default)]
    pub version_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Allocation {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub ip: String,
    #[serde(default)]
    pub port: i64,
    #[serde(default)]
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct DiamondInfo {
    #[serde(default)]
    pub left: i64,
    #[serde(default)]
    pub diamond_plan_id: i64,
    #[serde(default)]
    pub diamond_plan_discount: i64,
    #[serde(default)]
    pub diamond_plan_valid_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct TrafficInfo {
    #[serde(default)]
    pub plan: i64,
    #[serde(default)]
    pub remain_bytes: i64,
    #[serde(default)]
    pub auto_reset: bool,
    #[serde(default)]
    pub cd: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InsDetail {
    pub id: i64,
    pub name: Option<String>,
    #[serde(default)]
    pub is_pro: bool,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub is_suspended: bool,
    #[serde(default)]
    pub utilization: InsUtilization,
    #[serde(default)]
    pub game_info: GameInfo,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub last_paid_time: String,
    #[serde(default)]
    pub cpu: i64,
    #[serde(default)]
    pub disk: i64,
    #[serde(default)]
    pub ram: i64,
    #[serde(default)]
    pub point: i64,
    #[serde(default)]
    pub area_grade: String,
    #[serde(default)]
    pub shop_id: i64,
    #[serde(default)]
    pub default_allocation: Allocation,
    #[serde(default)]
    pub allocations: Vec<Allocation>,
    #[serde(default)]
    pub state: i32,
    #[serde(default)]
    pub have_task: bool,
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub dev_mode: bool,
    #[serde(default)]
    pub shortage: bool,
    #[serde(default)]
    pub diamond: DiamondInfo,
    #[serde(default)]
    pub traffic: TrafficInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileEntry {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub file: bool,
    #[serde(default)]
    pub size: Option<i64>,
    #[serde(default)]
    pub mime: Option<String>,
    #[serde(default)]
    pub modified_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InsFileListResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<FileEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InsFileContentResponse {
    pub code: i32,
    #[serde(default)]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SimpleMsgResponse {
    pub code: i32,
    #[serde(default)]
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChangeResponse {
    pub code: i32,
    #[serde(default)]
    pub msg: String,
    #[serde(default)]
    pub new_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct SupportData {
    #[serde(default)]
    pub dev_uid: i64,
    #[serde(default)]
    pub dev_qq: i64,
    #[serde(default)]
    pub support_group: i64,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub comment: String,
    #[serde(default)]
    pub valid: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SupportResponse {
    pub code: i32,
    #[serde(default)]
    pub data: SupportData,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GameListItem {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub pic_path: String,
    #[serde(default)]
    pub priority: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GameListResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<GameListItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GameKindItem {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub pic_path: String,
    #[serde(default)]
    pub priority: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GameKindListResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<GameKindItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GameVersionItem {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub priority: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GameVersionListResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<GameVersionItem>,
    #[serde(default)]
    pub is_windows: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CustomGameItem {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub is_windows: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub link: String,
    #[serde(default)]
    pub like: i64,
    #[serde(default)]
    pub dislike: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CustomGameListResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<CustomGameItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CustomVersionItem {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub recommend_setting: String,
    #[serde(default)]
    pub like: i64,
    #[serde(default)]
    pub dislike: i64,
    #[serde(default)]
    pub size: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CustomVersionListResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<CustomVersionItem>,
    #[serde(default)]
    pub is_windows: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BackupItem {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub status: i32,
    #[serde(default)]
    pub size: i64,
    #[serde(default)]
    pub valid_time: String,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub is_windows: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BackupListResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<BackupItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BackupDownloadResponse {
    pub code: i32,
    #[serde(default)]
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RollbackListResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ShopPlan {
    #[serde(default)]
    pub area_grade: String,
    #[serde(default)]
    pub area_is_windows: bool,
    #[serde(default)]
    pub area_vendor: String,
    #[serde(default)]
    pub cpu: i64,
    #[serde(default)]
    pub disk: i64,
    #[serde(default)]
    pub traffic: i64,
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub point: i64,
    #[serde(default)]
    pub ram: i64,
    #[serde(default)]
    pub spec: String,
    #[serde(default)]
    pub creatable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ShopListResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<ShopPlan>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct DiamondPlanItem {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub point_discount: i64,
    #[serde(default)]
    pub diamond: i64,
    #[serde(default)]
    pub shop_id: i64,
    #[serde(default)]
    pub grade: String,
    #[serde(default)]
    pub spec: String,
    #[serde(default)]
    pub cpu: i64,
    #[serde(default)]
    pub ram: i64,
    #[serde(default)]
    pub disk: i64,
    #[serde(default)]
    pub traffic: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiamondPlanResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<DiamondPlanItem>,
    #[serde(default)]
    pub diamond: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct SftpData {
    #[serde(default)]
    pub ip: String,
    #[serde(default)]
    pub port: String,
    #[serde(default)]
    pub user_name: String,
    #[serde(default)]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SftpResponse {
    pub code: i32,
    pub data: SftpData,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct TaskItem {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub status: i32,
    #[serde(default)]
    pub comment: String,
    #[serde(default)]
    pub create_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TasksResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<TaskItem>,
    #[serde(default)]
    pub running: i64,
    #[serde(default)]
    pub waiting: i64,
    #[serde(default)]
    pub num_first_waiting: i64,
    #[serde(default)]
    pub running_pro: i64,
    #[serde(default)]
    pub waiting_pro: i64,
    #[serde(default)]
    pub num_first_waiting_pro: i64,
    #[serde(default)]
    pub is_pro: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct StatItem {
    #[serde(default)]
    pub uptime: i64,
    #[serde(default)]
    pub in_bytes: i64,
    #[serde(default)]
    pub out_bytes: i64,
    #[serde(default)]
    pub cpu_percent: f64,
    #[serde(default)]
    pub mem_used_bytes: i64,
    #[serde(default)]
    pub create_time_timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StatListResponse {
    pub code: i32,
    #[serde(default)]
    pub list: Vec<StatItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DevImageItem {
    pub id: i64,
    pub name: String,
    pub state: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DevImageListResponse {
    pub code: i32,
    pub msg: Option<String>,
    pub list: Option<Vec<DevImageItem>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DevImageDetail {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub state: Option<bool>,
    pub install_count: Option<i64>,
    pub restore_times: Option<i64>,
    pub like: Option<i64>,
    pub dislike: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl DevImageDetail {
    pub fn installations(&self) -> i64 {
        self.install_count.or(self.restore_times).unwrap_or(0)
    }

    pub fn is_public(&self) -> bool {
        self.state == Some(true)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DevImageDetailResponse {
    pub code: i32,
    pub msg: Option<String>,
    pub data: Option<DevImageDetail>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DevImageVersion {
    pub id: Option<i64>,
    pub state: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DevImageVersionResponse {
    pub code: i32,
    pub msg: Option<String>,
    pub list: Option<Vec<DevImageVersion>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DevImageFeedback {
    pub id: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DevImageFeedbackResponse {
    pub code: i32,
    pub msg: Option<String>,
    pub list: Option<Vec<DevImageFeedback>>,
}

// 技术支持返回模型
// 单个支持记录的模型
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DevSupport {
    pub target_uid: String,        
    pub target_qq: Option<String>,   
    pub ins_id: String,
    pub create_time: String,          
    pub comment: String,
}

// 响应模型
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DevSupportListResponse {
    pub code: i32,                  
    pub list: Vec<DevSupport>,
}