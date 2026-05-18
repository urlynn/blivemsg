use serde_json::Value;
use crate::message::*;

#[cfg(feature = "protobuf-support")]
use base64::Engine as _;
#[cfg(feature = "protobuf-support")]
use bytes::Bytes;
#[cfg(feature = "protobuf-support")]
use prost::Message as ProstMessage;

// Protobuf message definitions for INTERACT_WORD_V2
#[cfg(feature = "protobuf-support")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractWordV2Proto {
    #[prost(uint64, tag = "1")]
    pub uid: u64,
    #[prost(string, tag = "2")]
    pub uname: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub uname_color: ::prost::alloc::string::String,
    #[prost(uint64, repeated, packed = "false", tag = "4")]
    pub identities: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag = "5")]
    pub msg_type: u64,
    #[prost(uint64, tag = "6")]
    pub roomid: u64,
    #[prost(uint64, tag = "7")]
    pub timestamp: u64,
    #[prost(uint64, tag = "8")]
    pub score: u64,
}

/// 将原始 JSON 转换为强类型 Message
pub fn parse_message(cmd: &str, json_value: &Value) -> Option<Message> {
    match cmd {
        // 用户消息
        "DANMU_MSG" => parse_danmu(json_value),
        "SEND_GIFT" => parse_gift(json_value),
        "COMBO_SEND" => parse_combo_send(json_value),
        "SUPER_CHAT_MESSAGE" | "SUPER_CHAT_MESSAGE_JPN" => parse_super_chat(json_value),
        "GUARD_BUY" => parse_guard_buy(json_value),
        "LIKE_INFO_V3_CLICK" => parse_like_info_v3_click(json_value),
        "WELCOME_GUARD" => parse_welcome_guard(json_value),
        "ENTRY_EFFECT" => parse_entry_effect(json_value),
        
        #[cfg(feature = "protobuf-support")]
        "INTERACT_WORD_V2" => parse_interact_word_v2(json_value),
        
        // 系统消息
        "USER_TOAST_MSG" => parse_user_toast_msg(json_value),
        "USER_TOAST_MSG_V2" => parse_user_toast_msg_v2(json_value),
        "NOTICE_MSG" => parse_notice_msg(json_value),
        "COMMON_NOTICE_DANMAKU" => parse_common_notice_danmaku(json_value),
        "WARNING" => parse_warning(json_value),
        "CUT_OFF" => parse_cut_off(json_value),
        "ROOM_BLOCK_MSG" => parse_room_block_msg(json_value),
        "ROOM_SILENT_OFF" => parse_room_silent_off(json_value),
        "ONLINE_RANK_V2" => parse_online_rank_v2(json_value),
        "ONLINE_RANK_V3" => Some(Message::OnlineRankV3(OnlineRankV3)),
        "ONLINE_RANK_COUNT" => parse_online_rank_count(json_value),
        "RANK_CHANGED" => parse_rank_changed(json_value),
        "RANK_CHANGED_V2" => parse_rank_changed_v2(json_value),
        "RANK_REM" => parse_rank_rem(json_value),
        "POPULAR_RANK_CHANGED" => parse_popular_rank_changed(json_value),
        "HOT_ROOM_NOTIFY" => parse_hot_room_notify(json_value),
        "WATCHED_CHANGE" => parse_watched_change(json_value),
        "POPULARITY_CHANGE" => parse_popularity_change(json_value),
        "LIVE" => Some(Message::Live(Live)),
        "PREPARING" => Some(Message::Preparing(Preparing)),
        "STOP_LIVE_ROOM_LIST" => Some(Message::StopLiveRoomList(StopLiveRoomList)),
        "ROOM_REAL_TIME_MESSAGE_UPDATE" => parse_room_real_time_message_update(json_value),
        "VOICE_JOIN_ROOM_COUNT_INFO" => parse_voice_join_room_count_info(json_value),
        "VOICE_JOIN_LIST" => parse_voice_join_list(json_value),
        "LIVE_MULTI_VIEW_NEW_INFO" => parse_live_multi_view_new_info(json_value),
        "ANCHOR_LOT_START" => parse_anchor_lot_start(json_value),
        "ANCHOR_LOT_END" => parse_anchor_lot_end(json_value),
        "ANCHOR_LOT_AWARD" => parse_anchor_lot_award(json_value),
        "SUPER_CHAT_ENTRANCE" => parse_super_chat_entrance(json_value),
        "RECALL_DANMU_MSG" => parse_recall_danmu_msg(json_value),
        "PLAYURL_RELOAD" => parse_playurl_reload(json_value),
        "PLAYURL_RELOAD_MASTER" => parse_playurl_reload_master(json_value),
        "SHOPPING_CART_SHOW" => parse_shopping_cart_show(json_value),
        "WIDGET_GIFT_STAR_PROCESS_V2" => parse_widget_gift_star_process_v2(json_value),
        "WIDGET_GIFT_STAR_PROCESS" => parse_widget_gift_star_process(json_value),
        "OTHER_SLICE_LOADING_RESULT" => parse_other_slice_loading_result(json_value),
        "INTERACTIVE_USER" => parse_interactive_user(json_value),
        "ONLINE_COUNT" => parse_online_count(json_value),
        "DM_INTERACTION" => parse_dm_interaction(json_value),
        "LIKE_INFO_V3_UPDATE" => parse_like_info_v3_update(json_value),
        
        _ => None,
    }
}

// ==================== 用户消息解析函数 ====================

fn parse_danmu(json_value: &Value) -> Option<Message> {
    let info = json_value.get("info")?.as_array()?;
    if info.len() < 3 {
        return None;
    }
    
    let content = info.get(1)?.as_str()?.to_string();
    let user_info = info.get(2)?.as_array()?;
    if user_info.len() < 2 {
        return None;
    }
    
    let username = user_info.get(1)?.as_str()?.to_string();
    let uid = user_info.first()?.as_u64().unwrap_or(0);
    
    let is_admin = user_info.get(2)
        .and_then(|v| v.as_u64())
        .map(|v| v == 1)
        .unwrap_or(false);
    
    let medal_level = info.get(3)
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.get(1))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;
    
    Some(Message::Danmu(Danmu {
        username,
        content,
        is_admin,
        medal_level,
        uid,
    }))
}

fn parse_gift(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let username = data.get("uname")?.as_str()?.to_string();
    let gift_name = data.get("giftName")
        .or_else(|| data.get("gift_name"))?
        .as_str()?
        .to_string();
    let num = data.get("num")?.as_u64()? as u32;
    let price = data.get("price")
        .and_then(|p| p.as_f64())
        .unwrap_or(0.0);
    let uid = data.get("uid")
        .and_then(|u| u.as_u64())
        .unwrap_or(0);
    let action = data.get("action")
        .and_then(|a| a.as_str())
        .unwrap_or("投喂")
        .to_string();
    
    Some(Message::Gift(Gift {
        username,
        gift_name,
        num,
        price,
        uid,
        action,
    }))
}

fn parse_combo_send(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let username = data.get("uname")?.as_str()?.to_string();
    let gift_name = data.get("gift_name")?.as_str()?.to_string();
    let combo_num = data.get("combo_num")?.as_u64()? as u32;
    let uid = data.get("uid")
        .and_then(|u| u.as_u64())
        .unwrap_or(0);
    let action = data.get("action")
        .and_then(|a| a.as_str())
        .unwrap_or("投喂")
        .to_string();
    
    Some(Message::ComboSend(ComboSend {
        username,
        gift_name,
        combo_num,
        uid,
        action,
    }))
}

fn parse_super_chat(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let user_info = data.get("user_info")?;
    let username = user_info.get("uname")?.as_str()?.to_string();
    let content = data.get("message")?.as_str()?.to_string();
    let price = data.get("price")?.as_u64()? as f64 / 100.0;
    let uid = user_info.get("uid")
        .and_then(|u| u.as_u64())
        .unwrap_or(0);
    
    Some(Message::SuperChat(SuperChat {
        username,
        content,
        price,
        uid,
    }))
}

fn parse_guard_buy(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let username = data.get("username")
        .or_else(|| data.get("uname"))?
        .as_str()?
        .to_string();
    let guard_level = data.get("guard_level")?.as_u64()? as u32;
    let num = data.get("num")?.as_u64()? as u32;
    let price = data.get("price")
        .and_then(|p| p.as_f64())
        .unwrap_or(0.0);
    let uid = data.get("uid")
        .and_then(|u| u.as_u64())
        .unwrap_or(0);
    
    Some(Message::GuardBuy(GuardBuy {
        username,
        guard_level,
        num,
        price,
        uid,
    }))
}

fn parse_like_info_v3_click(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let username = data.get("uname")?.as_str()?.to_string();
    let click_count = data.get("click_count")
        .and_then(|c| c.as_u64())
        .unwrap_or(0);
    let uid = data.get("uid")?.as_u64()?;
    
    Some(Message::LikeInfoV3Click(LikeInfoV3Click { 
        username,
        click_count,
        uid 
    }))
}
fn parse_welcome_guard(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let username = data.get("username")
        .or_else(|| data.get("uname"))?
        .as_str()?
        .to_string();
    let guard_level = data.get("guard_level")?.as_u64()? as u32;
    let uid = data.get("uid")
        .and_then(|u| u.as_u64())
        .unwrap_or(0);
    
    Some(Message::WelcomeGuard(WelcomeGuard {
        username,
        guard_level,
        uid,
    }))
}

fn parse_entry_effect(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let uid = data.get("uid")
        .and_then(|u| u.as_u64())
        .unwrap_or(0);
    let copy_writing = data.get("copy_writing_v2")
        .or_else(|| data.get("copy_writing"))?
        .as_str()?
        .to_string();
    
    Some(Message::EntryEffect(EntryEffect { 
        uid,
        copy_writing 
    }))
}

#[cfg(feature = "protobuf-support")]
fn parse_interact_word_v2(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let pb_field = data.get("pb")?.as_str()?;
    
    let decoded = base64::engine::general_purpose::STANDARD.decode(pb_field).ok()?;
    let interact_msg = InteractWordV2Proto::decode(Bytes::from(decoded)).ok()?;
    
    if interact_msg.uname.is_empty() {
        return None;
    }
    
    Some(Message::InteractWordV2(InteractWordV2 {
        username: interact_msg.uname,
        uid: interact_msg.uid,
        msg_type: interact_msg.msg_type as u32,
    }))
}

// ==================== 系统消息解析函数 ====================

fn parse_user_toast_msg(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let username = data.get("uname")
        .or_else(|| data.get("username"))
        .and_then(|u| u.as_str())
        .unwrap_or("")
        .to_string();
    let toast_msg = data.get("toast_msg")?.as_str()?.to_string();
    let uid = data.get("uid")
        .and_then(|u| u.as_u64())
        .unwrap_or(0);
    
    Some(Message::UserToastMsg(UserToastMsg {
        username,
        toast_msg,
        uid,
    }))
}

fn parse_user_toast_msg_v2(json_value: &Value) -> Option<Message> {
    parse_user_toast_msg(json_value)
}

fn parse_notice_msg(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let message = data.get("msg_self")
        .and_then(|m| m.as_str())
        .or_else(|| data.get("msg_common").and_then(|m| m.as_str()))?
        .to_string();
    
    Some(Message::NoticeMsg(NoticeMsg { message }))
}

fn parse_common_notice_danmaku(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let content_segments = data.get("content_segments")?.as_array()?;
    
    let mut content = String::new();
    for segment in content_segments {
        if let Some(text) = segment.get("text").and_then(|t| t.as_str()) {
            content.push_str(text);
        }
    }
    
    Some(Message::CommonNoticeDanmaku(CommonNoticeDanmaku { content }))
}

fn parse_warning(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let message = data.get("msg")?.as_str()?.to_string();
    
    Some(Message::Warning(Warning { message }))
}

fn parse_cut_off(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let message = data.get("msg")?.as_str()?.to_string();
    
    Some(Message::CutOff(CutOff { message }))
}

fn parse_room_block_msg(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let blocked_uid = data.get("block_uid")?.as_u64()?;
    let operator_uid = data.get("operator")?.as_u64()?;
    
    Some(Message::RoomBlockMsg(RoomBlockMsg {
        blocked_uid,
        operator_uid,
    }))
}

fn parse_room_silent_off(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let message = data.get("msg")?.as_str()?.to_string();
    
    Some(Message::RoomSilentOff(RoomSilentOff { message }))
}

fn parse_online_rank_v2(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let list = data.get("list")?.as_array()?;
    
    let mut rank_list = Vec::new();
    for item in list {
        if let Some(uname) = item.get("uname").and_then(|u| u.as_str())
            && let Some(score) = item.get("score").and_then(|s| s.as_str()) {
                rank_list.push(OnlineRankItem {
                    uname: uname.to_string(),
                    score: score.to_string(),
                });
            }
    }
    
    Some(Message::OnlineRankV2(OnlineRankV2 { list: rank_list }))
}

fn parse_online_rank_count(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let count = data.get("count")?.as_u64()?;
    
    Some(Message::OnlineRankCount(OnlineRankCount { count }))
}

fn parse_rank_changed(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let rank_name = data.get("rank_name_by_type")?.as_str()?.to_string();
    let rank = data.get("rank")?.as_u64()?;
    
    Some(Message::RankChanged(RankChanged { rank_name, rank }))
}

fn parse_rank_changed_v2(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let rank_name = data.get("rank_name_by_type")?.as_str()?.to_string();
    let rank = data.get("rank")?.as_u64()?;
    let on_rank_name = data.get("on_rank_name_by_type")?.as_str()?.to_string();
    
    Some(Message::RankChangedV2(RankChangedV2 {
        rank_name,
        rank,
        on_rank_name,
    }))
}

fn parse_rank_rem(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let name = data.get("name")?.as_str()?.to_string();
    let uid = data.get("uid")?.as_u64()?;
    
    Some(Message::RankRem(RankRem { name, uid }))
}

fn parse_popular_rank_changed(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let rank_name = data.get("rank_name_by_type")?.as_str()?.to_string();
    let rank = data.get("rank")?.as_u64()?;
    
    Some(Message::PopularRankChanged(PopularRankChanged { rank_name, rank }))
}

fn parse_hot_room_notify(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let threshold = data.get("threshold")?.as_u64()?;
    
    Some(Message::HotRoomNotify(HotRoomNotify { threshold }))
}

fn parse_watched_change(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let text_large = data.get("text_large")?.as_str()?.to_string();
    
    Some(Message::WatchedChange(WatchedChange { text_large }))
}

fn parse_popularity_change(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let popularity = data.get("popularity")?.as_u64()?;
    
    Some(Message::PopularityChange(PopularityChange { popularity }))
}

fn parse_room_real_time_message_update(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let roomid = data.get("roomid")?.as_u64()?;
    let fans = data.get("fans")?.as_u64()?;
    
    Some(Message::RoomRealTimeMessageUpdate(RoomRealTimeMessageUpdate {
        roomid,
        fans,
    }))
}

fn parse_voice_join_room_count_info(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let apply_count = data.get("apply_count")?.as_u64()?;
    let notify_count = data.get("notify_count")?.as_u64()?;
    
    Some(Message::VoiceJoinRoomCountInfo(VoiceJoinRoomCountInfo {
        apply_count,
        notify_count,
    }))
}

fn parse_voice_join_list(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let apply_count = data.get("apply_count")?.as_u64()?;
    let red_point = data.get("red_point")?.as_u64()?;
    
    Some(Message::VoiceJoinList(VoiceJoinList {
        apply_count,
        red_point,
    }))
}

fn parse_live_multi_view_new_info(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let title = data.get("title")?.as_str()?.to_string();
    
    Some(Message::LiveMultiViewNewInfo(LiveMultiViewNewInfo { title }))
}

fn parse_anchor_lot_start(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let lot_id = data.get("id")?.as_u64()?;
    
    Some(Message::AnchorLotStart(AnchorLotStart { lot_id }))
}

fn parse_anchor_lot_end(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let lot_id = data.get("id")?.as_u64()?;
    
    Some(Message::AnchorLotEnd(AnchorLotEnd { lot_id }))
}

fn parse_anchor_lot_award(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let award_name = data.get("award_name")?.as_str()?.to_string();
    let winner = data.get("award_users")
        .and_then(|users| users.as_array())
        .and_then(|arr| arr.first())
        .and_then(|user| user.get("uname"))
        .and_then(|name| name.as_str())
        .map(|s| s.to_string());
    
    Some(Message::AnchorLotAward(AnchorLotAward {
        award_name,
        winner,
    }))
}

fn parse_super_chat_entrance(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let status = data.get("status")?.as_u64()?;
    
    Some(Message::SuperChatEntrance(SuperChatEntrance { status }))
}

fn parse_recall_danmu_msg(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let recall_type = data.get("recall_type")?.as_u64()?;
    
    Some(Message::RecallDanmuMsg(RecallDanmuMsg { recall_type }))
}

fn parse_playurl_reload(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let room_id = data.get("room_id")?.as_u64()?;
    
    Some(Message::PlayurlReload(PlayurlReload { room_id }))
}

fn parse_playurl_reload_master(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let room_id = data.get("room_id")?.as_u64()?;
    
    Some(Message::PlayurlReloadMaster(PlayurlReloadMaster { room_id }))
}

fn parse_shopping_cart_show(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let status = data.get("status")?.as_u64()?;
    
    Some(Message::ShoppingCartShow(ShoppingCartShow { status }))
}

fn parse_widget_gift_star_process_v2(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let name = data.get("name")?.as_str()?.to_string();
    let cur_num = data.get("cur_num")?.as_u64()?;
    let total_num = data.get("total_num")?.as_u64()?;
    
    Some(Message::WidgetGiftStarProcessV2(WidgetGiftStarProcessV2 {
        name,
        cur_num,
        total_num,
    }))
}

fn parse_widget_gift_star_process(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let process_list = data.get("process_list")?.as_array()?;
    
    let mut processes = Vec::new();
    for process in process_list {
        if let Some(gift_name) = process.get("gift_name").and_then(|n| n.as_str())
            && let Some(completed_num) = process.get("completed_num").and_then(|c| c.as_u64())
                && let Some(target_num) = process.get("target_num").and_then(|t| t.as_u64()) {
                    processes.push(GiftStarProcess {
                        gift_name: gift_name.to_string(),
                        completed_num,
                        target_num,
                    });
                }
    }
    
    Some(Message::WidgetGiftStarProcess(WidgetGiftStarProcess { processes }))
}

fn parse_other_slice_loading_result(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let live_key = data.get("live_key")?.as_str()?.to_string();
    
    Some(Message::OtherSliceLoadingResult(OtherSliceLoadingResult { live_key }))
}

fn parse_interactive_user(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let value = data.get("value")?;
    let dm_msg = value.get("dm_msg")?.as_str()?.to_string();
    
    Some(Message::InteractiveUser(InteractiveUser { dm_msg }))
}

fn parse_online_count(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let count = data.get("count")?.as_u64()?;
    
    Some(Message::OnlineCount(OnlineCount { count }))
}

fn parse_dm_interaction(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let interaction_data_str = data.get("data")?.as_str()?;
    let interaction_json: Value = serde_json::from_str(interaction_data_str).ok()?;
    
    let cnt = interaction_json.get("cnt")?.as_u64()?;
    let suffix_text = interaction_json.get("suffix_text")?.as_str()?.to_string();
    
    Some(Message::DmInteraction(DmInteraction {
        cnt,
        suffix_text,
    }))
}

fn parse_like_info_v3_update(json_value: &Value) -> Option<Message> {
    let data = json_value.get("data")?;
    let click_count = data.get("click_count")?.as_u64()?;
    
    Some(Message::LikeInfoV3Update(LikeInfoV3Update { click_count }))
}
