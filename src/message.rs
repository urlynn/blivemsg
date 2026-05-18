/// 弹幕消息
#[derive(Debug, Clone)]
pub struct Danmu {
    pub username: String,
    pub content: String,
    pub is_admin: bool,
    pub medal_level: u32,
    pub uid: u64,
}

/// 礼物消息
#[derive(Debug, Clone)]
pub struct Gift {
    pub username: String,
    pub gift_name: String,
    pub num: u32,
    pub price: f64,
    pub uid: u64,
    pub action: String,
}

/// 连击礼物消息
#[derive(Debug, Clone)]
pub struct ComboSend {
    pub username: String,
    pub gift_name: String,
    pub combo_num: u32,
    pub uid: u64,
    pub action: String,
}

/// 超级留言（SC）消息
#[derive(Debug, Clone)]
pub struct SuperChat {
    pub username: String,
    pub content: String,
    pub price: f64,
    pub uid: u64,
}

/// 舰长购买消息
#[derive(Debug, Clone)]
pub struct GuardBuy {
    pub username: String,
    pub guard_level: u32,
    pub num: u32,
    pub price: f64,
    pub uid: u64,
}

/// 点赞点击消息
#[derive(Debug, Clone)]
pub struct LikeInfoV3Click {
    pub username: String,
    pub click_count: u64,
    pub uid: u64,
}

/// 欢迎总督消息
#[derive(Debug, Clone)]
pub struct WelcomeGuard {
    pub username: String,
    pub guard_level: u32,
    pub uid: u64,
}

/// 入场特效消息
#[derive(Debug, Clone)]
pub struct EntryEffect {
    pub uid: u64,
    pub copy_writing: String,
}

/// 互动词消息（需要 protobuf-support 特性）
#[cfg(feature = "protobuf-support")]
#[derive(Debug, Clone)]
pub struct InteractWordV2 {
    pub username: String,
    pub uid: u64,
    pub msg_type: u32,
}

/// 用户 Toast 消息
#[derive(Debug, Clone)]
pub struct UserToastMsg {
    pub username: String,
    pub toast_msg: String,
    pub uid: u64,
}

/// 通知消息
#[derive(Debug, Clone)]
pub struct NoticeMsg {
    pub message: String,
}

/// 通用通知弹幕
#[derive(Debug, Clone)]
pub struct CommonNoticeDanmaku {
    pub content: String,
}

/// 警告消息
#[derive(Debug, Clone)]
pub struct Warning {
    pub message: String,
}

/// 切断消息
#[derive(Debug, Clone)]
pub struct CutOff {
    pub message: String,
}

/// 房间禁言消息
#[derive(Debug, Clone)]
pub struct RoomBlockMsg {
    pub blocked_uid: u64,
    pub operator_uid: u64,
}

/// 房间静音关闭
#[derive(Debug, Clone)]
pub struct RoomSilentOff {
    pub message: String,
}

/// 在线排名 V2
#[derive(Debug, Clone)]
pub struct OnlineRankV2 {
    pub list: Vec<OnlineRankItem>,
}

#[derive(Debug, Clone)]
pub struct OnlineRankItem {
    pub uname: String,
    pub score: String,
}

/// 在线排名 V3
#[derive(Debug, Clone)]
pub struct OnlineRankV3;

/// 在线排名人数
#[derive(Debug, Clone)]
pub struct OnlineRankCount {
    pub count: u64,
}

/// 排名变更
#[derive(Debug, Clone)]
pub struct RankChanged {
    pub rank_name: String,
    pub rank: u64,
}

/// 排名变更 V2
#[derive(Debug, Clone)]
pub struct RankChangedV2 {
    pub rank_name: String,
    pub rank: u64,
    pub on_rank_name: String,
}

/// 排名移除
#[derive(Debug, Clone)]
pub struct RankRem {
    pub name: String,
    pub uid: u64,
}

/// 人气排名变更
#[derive(Debug, Clone)]
pub struct PopularRankChanged {
    pub rank_name: String,
    pub rank: u64,
}

/// 热门房间通知
#[derive(Debug, Clone)]
pub struct HotRoomNotify {
    pub threshold: u64,
}

/// 观看人数变更
#[derive(Debug, Clone)]
pub struct WatchedChange {
    pub text_large: String,
}

/// 人气值变更
#[derive(Debug, Clone)]
pub struct PopularityChange {
    pub popularity: u64,
}

/// 直播开始
#[derive(Debug, Clone)]
pub struct Live;

/// 直播准备中
#[derive(Debug, Clone)]
pub struct Preparing;

/// 直播结束
#[derive(Debug, Clone)]
pub struct StopLiveRoomList;

/// 房间实时消息更新
#[derive(Debug, Clone)]
pub struct RoomRealTimeMessageUpdate {
    pub roomid: u64,
    pub fans: u64,
}

/// 语音房间计数信息
#[derive(Debug, Clone)]
pub struct VoiceJoinRoomCountInfo {
    pub apply_count: u64,
    pub notify_count: u64,
}

/// 语音加入列表
#[derive(Debug, Clone)]
pub struct VoiceJoinList {
    pub apply_count: u64,
    pub red_point: u64,
}

/// 多视角直播信息
#[derive(Debug, Clone)]
pub struct LiveMultiViewNewInfo {
    pub title: String,
}

/// 抽奖活动开始
#[derive(Debug, Clone)]
pub struct AnchorLotStart {
    pub lot_id: u64,
}

/// 抽奖活动结束
#[derive(Debug, Clone)]
pub struct AnchorLotEnd {
    pub lot_id: u64,
}

/// 抽奖获奖
#[derive(Debug, Clone)]
pub struct AnchorLotAward {
    pub award_name: String,
    pub winner: Option<String>,
}

/// SC 入口状态
#[derive(Debug, Clone)]
pub struct SuperChatEntrance {
    pub status: u64,
}

/// 弹幕撤回
#[derive(Debug, Clone)]
pub struct RecallDanmuMsg {
    pub recall_type: u64,
}

/// 播放 URL 重载
#[derive(Debug, Clone)]
pub struct PlayurlReload {
    pub room_id: u64,
}

/// 主播放 URL 重载
#[derive(Debug, Clone)]
pub struct PlayurlReloadMaster {
    pub room_id: u64,
}

/// 购物车显示
#[derive(Debug, Clone)]
pub struct ShoppingCartShow {
    pub status: u64,
}

/// 礼物星进程 V2
#[derive(Debug, Clone)]
pub struct WidgetGiftStarProcessV2 {
    pub name: String,
    pub cur_num: u64,
    pub total_num: u64,
}

/// 礼物星进程
#[derive(Debug, Clone)]
pub struct WidgetGiftStarProcess {
    pub processes: Vec<GiftStarProcess>,
}

#[derive(Debug, Clone)]
pub struct GiftStarProcess {
    pub gift_name: String,
    pub completed_num: u64,
    pub target_num: u64,
}

/// 其他切片加载结果
#[derive(Debug, Clone)]
pub struct OtherSliceLoadingResult {
    pub live_key: String,
}

/// 互动用户
#[derive(Debug, Clone)]
pub struct InteractiveUser {
    pub dm_msg: String,
}

/// 在线人数
#[derive(Debug, Clone)]
pub struct OnlineCount {
    pub count: u64,
}

/// 弹幕互动
#[derive(Debug, Clone)]
pub struct DmInteraction {
    pub cnt: u64,
    pub suffix_text: String,
}

/// 点赞信息更新
#[derive(Debug, Clone)]
pub struct LikeInfoV3Update {
    pub click_count: u64,
}

/// 统一消息枚举
#[derive(Debug, Clone)]
pub enum Message {
    // 用户消息
    Danmu(Danmu),
    Gift(Gift),
    ComboSend(ComboSend),
    SuperChat(SuperChat),
    GuardBuy(GuardBuy),
    LikeInfoV3Click(LikeInfoV3Click),
    WelcomeGuard(WelcomeGuard),
    EntryEffect(EntryEffect),
    #[cfg(feature = "protobuf-support")]
    InteractWordV2(InteractWordV2),
    
    // 系统消息
    UserToastMsg(UserToastMsg),
    UserToastMsgV2(UserToastMsg),
    NoticeMsg(NoticeMsg),
    CommonNoticeDanmaku(CommonNoticeDanmaku),
    Warning(Warning),
    CutOff(CutOff),
    RoomBlockMsg(RoomBlockMsg),
    RoomSilentOff(RoomSilentOff),
    OnlineRankV2(OnlineRankV2),
    OnlineRankV3(OnlineRankV3),
    OnlineRankCount(OnlineRankCount),
    RankChanged(RankChanged),
    RankChangedV2(RankChangedV2),
    RankRem(RankRem),
    PopularRankChanged(PopularRankChanged),
    HotRoomNotify(HotRoomNotify),
    WatchedChange(WatchedChange),
    PopularityChange(PopularityChange),
    Live(Live),
    Preparing(Preparing),
    StopLiveRoomList(StopLiveRoomList),
    RoomRealTimeMessageUpdate(RoomRealTimeMessageUpdate),
    VoiceJoinRoomCountInfo(VoiceJoinRoomCountInfo),
    VoiceJoinList(VoiceJoinList),
    LiveMultiViewNewInfo(LiveMultiViewNewInfo),
    AnchorLotStart(AnchorLotStart),
    AnchorLotEnd(AnchorLotEnd),
    AnchorLotAward(AnchorLotAward),
    SuperChatEntrance(SuperChatEntrance),
    RecallDanmuMsg(RecallDanmuMsg),
    PlayurlReload(PlayurlReload),
    PlayurlReloadMaster(PlayurlReloadMaster),
    ShoppingCartShow(ShoppingCartShow),
    WidgetGiftStarProcessV2(WidgetGiftStarProcessV2),
    WidgetGiftStarProcess(WidgetGiftStarProcess),
    OtherSliceLoadingResult(OtherSliceLoadingResult),
    InteractiveUser(InteractiveUser),
    OnlineCount(OnlineCount),
    DmInteraction(DmInteraction),
    LikeInfoV3Update(LikeInfoV3Update),
    
    /// 原始消息（用于未解析的消息类型）
    Raw {
        cmd: String,
        data: serde_json::Value,
    },
}

impl Message {
    /// 获取消息的命令字
    pub fn cmd(&self) -> &str {
        match self {
            Message::Danmu(_) => "DANMU_MSG",
            Message::Gift(_) => "SEND_GIFT",
            Message::ComboSend(_) => "COMBO_SEND",
            Message::SuperChat(_) => "SUPER_CHAT_MESSAGE",
            Message::GuardBuy(_) => "GUARD_BUY",
            Message::LikeInfoV3Click(_) => "LIKE_INFO_V3_CLICK",
            Message::WelcomeGuard(_) => "WELCOME_GUARD",
            Message::EntryEffect(_) => "ENTRY_EFFECT",
            #[cfg(feature = "protobuf-support")]
            Message::InteractWordV2(_) => "INTERACT_WORD_V2",
            Message::UserToastMsg(_) => "USER_TOAST_MSG",
            Message::UserToastMsgV2(_) => "USER_TOAST_MSG_V2",
            Message::NoticeMsg(_) => "NOTICE_MSG",
            Message::CommonNoticeDanmaku(_) => "COMMON_NOTICE_DANMAKU",
            Message::Warning(_) => "WARNING",
            Message::CutOff(_) => "CUT_OFF",
            Message::RoomBlockMsg(_) => "ROOM_BLOCK_MSG",
            Message::RoomSilentOff(_) => "ROOM_SILENT_OFF",
            Message::OnlineRankV2(_) => "ONLINE_RANK_V2",
            Message::OnlineRankV3(_) => "ONLINE_RANK_V3",
            Message::OnlineRankCount(_) => "ONLINE_RANK_COUNT",
            Message::RankChanged(_) => "RANK_CHANGED",
            Message::RankChangedV2(_) => "RANK_CHANGED_V2",
            Message::RankRem(_) => "RANK_REM",
            Message::PopularRankChanged(_) => "POPULAR_RANK_CHANGED",
            Message::HotRoomNotify(_) => "HOT_ROOM_NOTIFY",
            Message::WatchedChange(_) => "WATCHED_CHANGE",
            Message::PopularityChange(_) => "POPULARITY_CHANGE",
            Message::Live(_) => "LIVE",
            Message::Preparing(_) => "PREPARING",
            Message::StopLiveRoomList(_) => "STOP_LIVE_ROOM_LIST",
            Message::RoomRealTimeMessageUpdate(_) => "ROOM_REAL_TIME_MESSAGE_UPDATE",
            Message::VoiceJoinRoomCountInfo(_) => "VOICE_JOIN_ROOM_COUNT_INFO",
            Message::VoiceJoinList(_) => "VOICE_JOIN_LIST",
            Message::LiveMultiViewNewInfo(_) => "LIVE_MULTI_VIEW_NEW_INFO",
            Message::AnchorLotStart(_) => "ANCHOR_LOT_START",
            Message::AnchorLotEnd(_) => "ANCHOR_LOT_END",
            Message::AnchorLotAward(_) => "ANCHOR_LOT_AWARD",
            Message::SuperChatEntrance(_) => "SUPER_CHAT_ENTRANCE",
            Message::RecallDanmuMsg(_) => "RECALL_DANMU_MSG",
            Message::PlayurlReload(_) => "PLAYURL_RELOAD",
            Message::PlayurlReloadMaster(_) => "PLAYURL_RELOAD_MASTER",
            Message::ShoppingCartShow(_) => "SHOPPING_CART_SHOW",
            Message::WidgetGiftStarProcessV2(_) => "WIDGET_GIFT_STAR_PROCESS_V2",
            Message::WidgetGiftStarProcess(_) => "WIDGET_GIFT_STAR_PROCESS",
            Message::OtherSliceLoadingResult(_) => "OTHER_SLICE_LOADING_RESULT",
            Message::InteractiveUser(_) => "INTERACTIVE_USER",
            Message::OnlineCount(_) => "ONLINE_COUNT",
            Message::DmInteraction(_) => "DM_INTERACTION",
            Message::LikeInfoV3Update(_) => "LIKE_INFO_V3_UPDATE",
            Message::Raw { cmd, .. } => cmd.as_str(),
        }
    }
}
