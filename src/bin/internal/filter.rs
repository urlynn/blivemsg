use std::collections::HashSet;

/// 消息类型分类常量
pub const USER_MESSAGE_TYPES: &[&str] = &[
    "DANMU_MSG",
    #[cfg(feature = "protobuf-support")]
    "INTERACT_WORD_V2",
    "SEND_GIFT",
    "SUPER_CHAT_MESSAGE",
    "SUPER_CHAT_MESSAGE_JPN",
    "COMBO_SEND",
    "GUARD_BUY",
    "WELCOME_GUARD",
    "USER_TOAST_MSG",
    "USER_TOAST_MSG_V2",
    "LIKE_INFO_V3_CLICK",
    "ENTRY_EFFECT",
];

pub const SYSTEM_MESSAGE_TYPES: &[&str] = &[
    "NOTICE_MSG",
    "COMMON_NOTICE_DANMAKU",
    "WARNING",
    "CUT_OFF",
    "ROOM_BLOCK_MSG",
    "ROOM_SILENT_OFF",
    "ONLINE_RANK_V2",
    "ONLINE_RANK_V3",
    "ONLINE_RANK_COUNT",
    "RANK_CHANGED",
    "RANK_CHANGED_V2",
    "RANK_REM",
    "POPULAR_RANK_CHANGED",
    "HOT_ROOM_NOTIFY",
    "WATCHED_CHANGE",
    "POPULARITY_CHANGE",
    "LIVE",
    "PREPARING",
    "STOP_LIVE_ROOM_LIST",
    "ROOM_REAL_TIME_MESSAGE_UPDATE",
    "VOICE_JOIN_ROOM_COUNT_INFO",
    "VOICE_JOIN_LIST",
    "LIVE_MULTI_VIEW_NEW_INFO",
    "ANCHOR_LOT_START",
    "ANCHOR_LOT_END",
    "ANCHOR_LOT_AWARD",
    "SUPER_CHAT_ENTRANCE",
    "RECALL_DANMU_MSG",
    "PLAYURL_RELOAD",
    "PLAYURL_RELOAD_MASTER",
    "SHOPPING_CART_SHOW",
    "WIDGET_GIFT_STAR_PROCESS",
    "WIDGET_GIFT_STAR_PROCESS_V2",
    "OTHER_SLICE_LOADING_RESULT",
    "INTERACTIVE_USER",
    "ONLINE_COUNT",
    "DM_INTERACTION",
    "LIKE_INFO_V3_UPDATE",
];

/// 过滤模式
#[derive(Debug, Clone)]
enum FilterMode {
    User,      // 用户消息
    System,    // 系统消息
    All,       // 全部
    Specific,  // 具体类型
}

/// 消息类型过滤器
#[derive(Debug, Clone)]
pub struct MessageFilter {
    mode: FilterMode,
    specific_types: HashSet<String>,
}

impl MessageFilter {
    /// 从消息类型列表创建过滤器
    pub fn new(types: Option<Vec<String>>) -> Self {
        let types = match types {
            Some(t) if !t.is_empty() => t,
            _ => return Self::default(), // 默认显示用户消息
        };
        
        // 检查是否是预设类别
        if types.len() == 1 {
            match types[0].to_uppercase().as_str() {
                "USER" => return Self {
                    mode: FilterMode::User,
                    specific_types: HashSet::new(),
                },
                "SYSTEM" => return Self {
                    mode: FilterMode::System,
                    specific_types: HashSet::new(),
                },
                "ALL" => return Self {
                    mode: FilterMode::All,
                    specific_types: HashSet::new(),
                },
                _ => {}
            }
        }
        
        // 具体类型列表
        let specific_types = types.into_iter().collect();
        
        Self {
            mode: FilterMode::Specific,
            specific_types,
        }
    }
    
    /// 检查是否应该显示该消息
    pub fn should_show(&self, cmd: &str) -> bool {
        match self.mode {
            FilterMode::User => USER_MESSAGE_TYPES.contains(&cmd),
            FilterMode::System => SYSTEM_MESSAGE_TYPES.contains(&cmd),
            FilterMode::All => true,
            FilterMode::Specific => self.specific_types.contains(cmd),
        }
    }
}

impl Default for MessageFilter {
    fn default() -> Self {
        Self {
            mode: FilterMode::User,
            specific_types: HashSet::new(),
        }
    }
}
