# 消息类型列表

blivemsg 支持 52 种 B站直播消息类型。

## 用户消息（9种）

| 消息类型 | 结构体 | 说明 |
|---------|--------|------|
| DANMU_MSG | `Danmu` | 弹幕消息 |
| SEND_GIFT | `Gift` | 赠送礼物 |
| COMBO_SEND | `ComboSend` | 礼物连击 |
| SUPER_CHAT_MESSAGE | `SuperChat` | 醒目留言（SC） |
| GUARD_BUY | `GuardBuy` | 开通舰长/提督/总督 |
| LIKE_INFO_V3_CLICK | `LikeInfoV3Click` | 用户点赞 |
| WELCOME_GUARD | `WelcomeGuard` | 欢迎总督/提督进入直播间 |
| ENTRY_EFFECT | `EntryEffect` | 入场特效 |
| INTERACT_WORD_V2 | `InteractWordV2` | 进入直播间（需 `protobuf-support` feature） |

## 系统消息（43种）

| 消息类型 | 结构体 | 说明 |
|---------|--------|------|
| USER_TOAST_MSG | `UserToastMsg` | 用户 Toast 消息 |
| USER_TOAST_MSG_V2 | `UserToastMsgV2` | 用户 Toast 消息 V2 |
| NOTICE_MSG | `NoticeMsg` | 广播消息 |
| COMMON_NOTICE_DANMAKU | `CommonNoticeDanmaku` | 普通通知弹幕 |
| WARNING | `Warning` | 直播警告 |
| CUT_OFF | `CutOff` | 直播强制切断 |
| ROOM_BLOCK_MSG | `RoomBlockMsg` | 房间禁言消息 |
| ROOM_SILENT_OFF | `RoomSilentOff` | 房间禁言关闭 |
| ONLINE_RANK_V2 | `OnlineRankV2` | 在线排名 V2 |
| ONLINE_RANK_V3 | `OnlineRankV3` | 在线排名 V3 |
| ONLINE_RANK_COUNT | `OnlineRankCount` | 在线排名人数 |
| RANK_CHANGED | `RankChanged` | 排名变更 |
| RANK_CHANGED_V2 | `RankChangedV2` | 排名变更 V2 |
| RANK_REM | `RankRem` | 排名移除 |
| POPULAR_RANK_CHANGED | `PopularRankChanged` | 人气排名变更 |
| HOT_ROOM_NOTIFY | `HotRoomNotify` | 热门房间通知 |
| WATCHED_CHANGE | `WatchedChange` | 观看人数变更 |
| POPULARITY_CHANGE | `PopularityChange` | 人气值变更 |
| LIVE | `Live` | 直播开始 |
| PREPARING | `Preparing` | 直播准备中 |
| STOP_LIVE_ROOM_LIST | `StopLiveRoomList` | 直播结束 |
| ROOM_REAL_TIME_MESSAGE_UPDATE | `RoomRealTimeMessageUpdate` | 房间数据实时更新 |
| VOICE_JOIN_ROOM_COUNT_INFO | `VoiceJoinRoomCountInfo` | 语音申请信息 |
| VOICE_JOIN_LIST | `VoiceJoinList` | 语音列表 |
| LIVE_MULTI_VIEW_NEW_INFO | `LiveMultiViewNewInfo` | 多视角直播信息 |
| ANCHOR_LOT_START | `AnchorLotStart` | 抽奖活动开始 |
| ANCHOR_LOT_END | `AnchorLotEnd` | 抽奖活动结束 |
| ANCHOR_LOT_AWARD | `AnchorLotAward` | 抽奖奖品 |
| SUPER_CHAT_ENTRANCE | `SuperChatEntrance` | SC 入口状态 |
| RECALL_DANMU_MSG | `RecallDanmuMsg` | 弹幕撤回 |
| PLAYURL_RELOAD | `PlayurlReload` | 播放 URL 重载 |
| PLAYURL_RELOAD_MASTER | `PlayurlReloadMaster` | 主播放 URL 重载 |
| SHOPPING_CART_SHOW | `ShoppingCartShow` | 购物车状态 |
| WIDGET_GIFT_STAR_PROCESS_V2 | `WidgetGiftStarProcessV2` | 礼物星进度 V2 |
| WIDGET_GIFT_STAR_PROCESS | `WidgetGiftStarProcess` | 礼物星进度 |
| OTHER_SLICE_LOADING_RESULT | `OtherSliceLoadingResult` | 切片加载完成 |
| INTERACTIVE_USER | `InteractiveUser` | 互动用户 |
| ONLINE_COUNT | `OnlineCount` | 在线人数 |
| DM_INTERACTION | `DmInteraction` | 互动消息 |
| LIKE_INFO_V3_UPDATE | `LikeInfoV3Update` | 点赞数更新 |

## 原始消息

未解析的消息通过 `Message::Raw { cmd, data }` 传递：

```rust
Message::Raw {
    cmd: String,  // 消息命令
    data: serde_json::Value,  // 原始 JSON 数据
}
```
