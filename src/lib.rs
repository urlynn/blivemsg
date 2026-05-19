//! # blivemsg - Bilibili Live Message Library
//!
//! 一个轻量高效的异步B站直播消息客户端，支持 52+ B站直播消息类型。
//!
//! ## 快速开始
//!
//! ### Stream 模式（推荐）
//!
//! ```no_run
//! use blivemsg::{BliveClient, types::Message};
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), blivemsg::Error> {
//!     let mut client = BliveClient::new(7734200, "cookies.json")?;
//!     let mut stream = client.stream().await?;
//!
//!     while let Some(message) = stream.next().await {
//!         match message {
//!             Message::Danmu(d) => println!("[{}] {}: {}", d.medal_level, d.username, d.content),
//!             Message::Gift(g) => println!("{} {}了 {} x{}", g.username, g.action ,g.gift_name, g.num),
//!             _ => {}
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ### 回调模式
//!
//! ```no_run
//! use blivemsg::BliveClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), blivemsg::Error> {
//!     let mut client = BliveClient::from_cookie_string(7734200, "SESSDATA=xxx;buvid3=yyy".to_string())?;
//!
//!     client.on_danmu(|danmu| {
//!         println!("[{}] {}: {}", danmu.medal_level, danmu.username, danmu.content);
//!     }).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## 架构说明
//!
//! - **[BliveClient]**: 核心客户端，负责建立和管理WebSocket连接
//! - **[types::Message]**: 消息枚举，包含所有52+种消息类型
//! - **[Error]**: 统一错误类型
//!
//! ## 更多资源
//!
//! - **[GitHub](https://github.com/urlynn/blivemsg)** - 源码仓库、Issue 反馈
//! - **[完整消息列表](https://github.com/urlynn/blivemsg/blob/main/MESSAGES.md)** - 52+种消息详细说明

mod client;
mod message;
mod error;
mod internal;

// ============================================================================
// 公共 API 导出
// ============================================================================

/// 核心客户端
pub use client::BliveClient;

// 消息类型定义
pub mod types {
    //! 所有消息类型定义
    //!
    //! 本模块包含52种消息类型，分为用户消息和系统消息两大类.
    
    pub use crate::message::Message;
    
    // --- 用户消息 (9种) ---
    pub use crate::message::{
        Danmu, Gift, ComboSend, SuperChat, GuardBuy, 
        WelcomeGuard, LikeInfoV3Click, EntryEffect,
    };
    
    #[cfg(feature = "protobuf-support")]
    pub use crate::message::InteractWordV2;
    
    // --- 系统消息 (38种) ---
    pub use crate::message::{
        UserToastMsg, NoticeMsg, CommonNoticeDanmaku, Warning, CutOff, RoomBlockMsg,
        RoomSilentOff, OnlineRankV2, OnlineRankV3, OnlineRankCount,
        RankChanged, RankChangedV2, RankRem, PopularRankChanged,
        HotRoomNotify, WatchedChange, PopularityChange, Live, Preparing,
        StopLiveRoomList, RoomRealTimeMessageUpdate, VoiceJoinRoomCountInfo,
        VoiceJoinList, LiveMultiViewNewInfo, AnchorLotStart, AnchorLotEnd,
        AnchorLotAward, SuperChatEntrance, RecallDanmuMsg, PlayurlReload,
        PlayurlReloadMaster, ShoppingCartShow, WidgetGiftStarProcessV2,
        WidgetGiftStarProcess, OtherSliceLoadingResult, InteractiveUser,
        OnlineCount, DmInteraction, LikeInfoV3Update,
    };
}

/// 错误类型
pub use error::Error;