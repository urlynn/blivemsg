use blivemsg::{BliveClient, types::Message};
use clap::Parser;
use futures_util::StreamExt;
use serde_json::Value;

mod internal {
    pub mod filter;
    pub mod config;
}
use internal::filter::MessageFilter;
use internal::config::{CliConfig, CookieSource};

/// 从 B站 API 获取用户信息
async fn get_user_info_from_api(cookies: &str) -> Result<(u64, String), Box<dyn std::error::Error>> {
    #[cfg(feature = "http-wreq")]
    let client = wreq::Client::new();
    
    #[cfg(feature = "http-reqwest")]
    let client = reqwest::Client::new();
    
    let url = "https://api.bilibili.com/x/web-interface/nav";
    
    let resp = client.get(url)
        .header("Cookie", cookies)
        .send()
        .await?;
    
    let text = resp.text().await?;
    let json: Value = serde_json::from_str(&text)?;
    
    let uid = json["data"]["mid"]
        .as_u64()
        .ok_or("无法获取 UID")?;
    
    let uname = json["data"]["uname"]
        .as_str()
        .ok_or("无法获取用户名")?
        .to_string();
    
    Ok((uid, uname))
}

#[derive(Parser, Debug)]
#[command(name = "blivemsg-cli")]
#[command(about = "B站直播弹幕客户端")]
struct Args {
    /// 直播间ID
    #[arg(short = 'r', long, default_value = "3151244")]
    room_id: u64,

    /// Cookie（JSON文件路径、JSON字符串或内联字符串）
    /// 
    /// 示例:
    ///   --cookie cookies.json                           # JSON 文件
    ///   --cookie '{"SESSDATA":"xxx","buvid3":"xxx"}'   # JSON 字符串
    ///   --cookie "SESSDATA=xxx;buvid3=xxx"             # 内联字符串
    #[arg(short = 'c', long, required_unless_present = "config")]
    cookie: Option<String>,

    /// 消息类型筛选
    /// 
    /// 预设类别: user(默认), system, all
    /// 具体类型: DANMU_MSG,SEND_GIFT 等（逗号分隔）
    #[arg(short = 't', long, value_delimiter = ',', default_value = "user")]
    message_types: Vec<String>,

    /// 静默模式（纯文本输出，不显示启动提示）
    /// 
    /// 适合管道处理和日志记录
    #[arg(short = 'q', long)]
    quiet: bool,

    /// 配置文件路径（TOML格式）
    /// 
    /// 配置文件可以包含 room_id, cookie, message_types 等参数
    /// 命令行参数优先级高于配置文件
    #[arg(short = 'C', long)]
    config: Option<String>,
}

/// 打印带颜色的消息（默认模式）
fn print_colored_message(message: &Message) {
    match message {
        Message::Danmu(d) => {
            println!("\x1b[32m💬 {}: {}\x1b[0m", d.username, d.content);
        }
        Message::Gift(g) => {
            println!("\x1b[35m🎁 {} {}了 {} x{}\x1b[0m", g.username, g.action, g.gift_name, g.num);
        }
        Message::SuperChat(sc) => {
            println!("\x1b[33m💎 SC [¥{:.2}] {}: {}\x1b[0m", sc.price, sc.username, sc.content);
        }
        Message::GuardBuy(g) => {
            let level_name = match g.guard_level {
                1 => "总督",
                2 => "提督",
                3 => "舰长",
                _ => "未知等级"
            };
            println!("\x1b[31m⚓ {} 开通了 {} x{}\x1b[0m", g.username, level_name, g.num);
        }
        Message::WelcomeGuard(w) => {
            let level_name = match w.guard_level {
                1 => "总督",
                2 => "提督",
                3 => "舰长",
                _ => "舰长"
            };
            println!("\x1b[36m👑 欢迎{} {} 进入直播间\x1b[0m", level_name, w.username);
        }
        Message::ComboSend(c) => {
            println!("\x1b[35m🎯 {} {}了{} x{}\x1b[0m", c.username, c.action ,c.gift_name, c.combo_num);
        }
        Message::UserToastMsg(t) | Message::UserToastMsgV2(t) => {
            println!("\x1b[34m {}\x1b[0m", t.toast_msg);
        }
        Message::LikeInfoV3Click(l) => {
            println!("👍 用户 {} 点赞了", l.username);
        }
        Message::EntryEffect(e) => {
            println!("\x1b[36m✨ {}\x1b[0m", e.copy_writing);
        }
        #[cfg(feature = "protobuf-support")]
        Message::InteractWordV2(i) => {
            println!("\x1b[36m👋 {} 进入了直播间\x1b[0m", i.username);
        }
        
        Message::NoticeMsg(n) => {
            println!("\x1b[90m 广播: {}\x1b[0m", n.message);
        }
        Message::CommonNoticeDanmaku(c) => {
            println!("\x1b[90m📢 通知: {}\x1b[0m", c.content);
        }
        Message::Warning(w) => {
            println!("\x1b[33m⚠️ 直播警告: {}\x1b[0m", w.message);
        }
        Message::CutOff(c) => {
            println!("\x1b[31m🛑 直播强制切断: {}\x1b[0m", c.message);
        }
        Message::RoomBlockMsg(r) => {
            println!("\x1b[31m🔇 用户 {} 被 {} 禁言\x1b[0m", r.blocked_uid, r.operator_uid);
        }
        Message::RoomSilentOff(r) => {
            println!("\x1b[90m{}\x1b[0m", r.message);
        }
        Message::OnlineRankV2(o) => {
            println!("\x1b[90m🏆 在线排名V2 (前{}名):\x1b[0m", o.list.len());
            for (i, item) in o.list.iter().take(5).enumerate() {
                println!("  {}. {} (分数: {})", i + 1, item.uname, item.score);
            }
        }
        Message::OnlineRankV3(_) => {
            println!("\x1b[90m🏆 在线排名更新\x1b[0m");
        }
        Message::OnlineRankCount(o) => {
            println!("\x1b[90m🏆 在线排名人数: {}\x1b[0m", o.count);
        }
        Message::RankChanged(r) => {
            println!("\x1b[90m🏆 排名变更: {} (第{}名)\x1b[0m", r.rank_name, r.rank);
        }
        Message::RankChangedV2(r) => {
            println!("\x1b[90m {}排名变更: {} (第{}名)\x1b[0m", r.on_rank_name, r.rank_name, r.rank);
        }
        Message::RankRem(r) => {
            println!("\x1b[90m🏆 排名移除: {} (用户ID: {})\x1b[0m", r.name, r.uid);
        }
        Message::PopularRankChanged(p) => {
            println!("\x1b[90m🏆 人气排名变更: {} (第{}名)\x1b[0m", p.rank_name, p.rank);
        }
        Message::HotRoomNotify(h) => {
            println!("\x1b[90m🔥 热门房间通知 - 阈值: {}\x1b[0m", h.threshold);
        }
        Message::WatchedChange(w) => {
            println!("\x1b[90m👀 {}\x1b[0m", w.text_large);
        }
        Message::PopularityChange(p) => {
            println!("\x1b[90m 人气值变更: {}\x1b[0m", p.popularity);
        }
        Message::Live(_) => {
            println!("\x1b[32m📺 直播开始\x1b[0m");
        }
        Message::Preparing(_) => {
            println!("\x1b[33m⏳ 直播准备中\x1b[0m");
        }
        Message::StopLiveRoomList(_) => {
            println!("\x1b[31m 直播结束\x1b[0m");
        }
        Message::RoomRealTimeMessageUpdate(r) => {
            println!("\x1b[90m📊 房间 {} 数据更新 - 粉丝数: {}\x1b[0m", r.roomid, r.fans);
        }
        Message::VoiceJoinRoomCountInfo(v) => {
            println!("\x1b[90m🎤 语音申请: {} 通知: {}\x1b[0m", v.apply_count, v.notify_count);
        }
        Message::VoiceJoinList(v) => {
            println!("\x1b[90m🎤 语音列表: 申请 {} 红点 {}\x1b[0m", v.apply_count, v.red_point);
        }
        Message::LiveMultiViewNewInfo(l) => {
            println!("\x1b[90m📺 多视角直播: {}\x1b[0m", l.title);
        }
        Message::AnchorLotStart(a) => {
            println!("\x1b[33m🎰 抽奖活动 {} 已开始\x1b[0m", a.lot_id);
        }
        Message::AnchorLotEnd(a) => {
            println!("\x1b[33m 抽奖活动 {} 已结束\x1b[0m", a.lot_id);
        }
        Message::AnchorLotAward(a) => {
            if let Some(winner) = &a.winner {
                println!("\x1b[35m {} 获得了 {}\x1b[0m", winner, a.award_name);
            } else {
                println!("\x1b[35m🎁 抽奖奖品: {}\x1b[0m", a.award_name);
            }
        }
        Message::SuperChatEntrance(s) => {
            if s.status == 0 {
                println!("\x1b[33m💎 SC 入口已开启\x1b[0m");
            } else {
                println!("\x1b[90m💎 SC 入口已关闭\x1b[0m");
            }
        }
        Message::RecallDanmuMsg(r) => {
            println!("\x1b[90m🗑️ 弹幕撤回 (类型: {})\x1b[0m", r.recall_type);
        }
        Message::PlayurlReload(p) => {
            println!("\x1b[90m🔄 播放URL重载: 房间 {}\x1b[0m", p.room_id);
        }
        Message::PlayurlReloadMaster(p) => {
            println!("\x1b[90m🔄 主播放URL重载: 房间 {}\x1b[0m", p.room_id);
        }
        Message::ShoppingCartShow(s) => {
            println!("\x1b[90m🛒 购物车状态: {}\x1b[0m", s.status);
        }
        Message::WidgetGiftStarProcessV2(w) => {
            println!("\x1b[35m⭐ {} 进度: {}/{}\x1b[0m", w.name, w.cur_num, w.total_num);
        }
        Message::WidgetGiftStarProcess(w) => {
            for process in &w.processes {
                println!("\x1b[35m⭐ {} 进度: {}/{}\x1b[0m", process.gift_name, process.completed_num, process.target_num);
            }
        }
        Message::OtherSliceLoadingResult(o) => {
            println!("\x1b[90m📼 切片加载完成: {}\x1b[0m", o.live_key);
        }
        Message::InteractiveUser(i) => {
            println!("\x1b[90m🎮 互动: {}\x1b[0m", i.dm_msg);
        }
        Message::OnlineCount(o) => {
            println!("\x1b[90m 在线人数: {}\x1b[0m", o.count);
        }
        Message::DmInteraction(d) => {
            println!("\x1b[90m💬 {}{}\x1b[0m", d.cnt, d.suffix_text);
        }
        Message::LikeInfoV3Update(l) => {
            println!("\x1b[90m👍 总点赞数: {}\x1b[0m", l.click_count);
        }
        
        Message::Raw { .. } => {}
    }
}

/// 打印纯文本消息（静默模式）
fn print_plain_message(message: &Message) {
    match message {
        Message::Danmu(d) => {
            println!("{}: {}", d.username, d.content);
        }
        Message::Gift(g) => {
            println!("{} 送出了 {} x{}", g.username, g.gift_name, g.num);
        }
        Message::SuperChat(sc) => {
            println!("SC [¥{:.2}] {}: {}", sc.price, sc.username, sc.content);
        }
        Message::GuardBuy(g) => {
            let level_name = match g.guard_level {
                1 => "总督",
                2 => "提督",
                3 => "舰长",
                _ => "未知等级"
            };
            println!("{} 开通了 {} x{}", g.username, level_name, g.num);
        }
        Message::WelcomeGuard(w) => {
            let level_name = match w.guard_level {
                1 => "总督",
                2 => "提督",
                3 => "舰长",
                _ => "舰长"
            };
            println!("欢迎{} {} 进入直播间", level_name, w.username);
        }
        Message::ComboSend(c) => {
            println!("{} 连击 {} x{}", c.username, c.gift_name, c.combo_num);
        }
        Message::UserToastMsg(t) | Message::UserToastMsgV2(t) => {
            println!("{}", t.toast_msg);
        }
        Message::LikeInfoV3Click(l) => {
            println!("用户 {} 点赞了", l.uid);
        }
        Message::EntryEffect(e) => {
            println!("{}", e.copy_writing);
        }
        #[cfg(feature = "protobuf-support")]
        Message::InteractWordV2(i) => {
            println!("{} 进入了直播间", i.username);
        }
        
        Message::NoticeMsg(n) => {
            println!("广播: {}", n.message);
        }
        Message::CommonNoticeDanmaku(c) => {
            println!("通知: {}", c.content);
        }
        Message::Warning(w) => {
            println!("直播警告: {}", w.message);
        }
        Message::CutOff(c) => {
            println!("直播强制切断: {}", c.message);
        }
        Message::RoomBlockMsg(r) => {
            println!("用户 {} 被 {} 禁言", r.blocked_uid, r.operator_uid);
        }
        Message::RoomSilentOff(r) => {
            println!("{}", r.message);
        }
        Message::OnlineRankV2(o) => {
            println!("在线排名V2 (前{}名):", o.list.len());
            for (i, item) in o.list.iter().take(5).enumerate() {
                println!("  {}. {} (分数: {})", i + 1, item.uname, item.score);
            }
        }
        Message::OnlineRankV3(_) => {
            println!("在线排名更新");
        }
        Message::OnlineRankCount(o) => {
            println!("在线排名人数: {}", o.count);
        }
        Message::RankChanged(r) => {
            println!("排名变更: {} (第{}名)", r.rank_name, r.rank);
        }
        Message::RankChangedV2(r) => {
            println!("{}排名变更: {} (第{}名)", r.on_rank_name, r.rank_name, r.rank);
        }
        Message::RankRem(r) => {
            println!("排名移除: {} (用户ID: {})", r.name, r.uid);
        }
        Message::PopularRankChanged(p) => {
            println!("人气排名变更: {} (第{}名)", p.rank_name, p.rank);
        }
        Message::HotRoomNotify(h) => {
            println!("热门房间通知 - 阈值: {}", h.threshold);
        }
        Message::WatchedChange(w) => {
            println!("{}", w.text_large);
        }
        Message::PopularityChange(p) => {
            println!("人气值变更: {}", p.popularity);
        }
        Message::Live(_) => {
            println!("直播开始");
        }
        Message::Preparing(_) => {
            println!("直播准备中");
        }
        Message::StopLiveRoomList(_) => {
            println!("直播结束");
        }
        Message::RoomRealTimeMessageUpdate(r) => {
            println!("房间 {} 数据更新 - 粉丝数: {}", r.roomid, r.fans);
        }
        Message::VoiceJoinRoomCountInfo(v) => {
            println!("语音申请: {} 通知: {}", v.apply_count, v.notify_count);
        }
        Message::VoiceJoinList(v) => {
            println!("语音列表: 申请 {} 红点 {}", v.apply_count, v.red_point);
        }
        Message::LiveMultiViewNewInfo(l) => {
            println!("多视角直播: {}", l.title);
        }
        Message::AnchorLotStart(a) => {
            println!("抽奖活动 {} 已开始", a.lot_id);
        }
        Message::AnchorLotEnd(a) => {
            println!("抽奖活动 {} 已结束", a.lot_id);
        }
        Message::AnchorLotAward(a) => {
            if let Some(winner) = &a.winner {
                println!("{} 获得了 {}", winner, a.award_name);
            } else {
                println!("抽奖奖品: {}", a.award_name);
            }
        }
        Message::SuperChatEntrance(s) => {
            if s.status == 0 {
                println!("SC 入口已开启");
            } else {
                println!("SC 入口已关闭");
            }
        }
        Message::RecallDanmuMsg(r) => {
            println!("弹幕撤回 (类型: {})", r.recall_type);
        }
        Message::PlayurlReload(p) => {
            println!("播放URL重载: 房间 {}", p.room_id);
        }
        Message::PlayurlReloadMaster(p) => {
            println!("主播放URL重载: 房间 {}", p.room_id);
        }
        Message::ShoppingCartShow(s) => {
            println!("购物车状态: {}", s.status);
        }
        Message::WidgetGiftStarProcessV2(w) => {
            println!("{} 进度: {}/{}", w.name, w.cur_num, w.total_num);
        }
        Message::WidgetGiftStarProcess(w) => {
            for process in &w.processes {
                println!("{} 进度: {}/{}", process.gift_name, process.completed_num, process.target_num);
            }
        }
        Message::OtherSliceLoadingResult(o) => {
            println!("切片加载完成: {}", o.live_key);
        }
        Message::InteractiveUser(i) => {
            println!("互动: {}", i.dm_msg);
        }
        Message::OnlineCount(o) => {
            println!("在线人数: {}", o.count);
        }
        Message::DmInteraction(d) => {
            println!("{}{}", d.cnt, d.suffix_text);
        }
        Message::LikeInfoV3Update(l) => {
            println!("总点赞数: {}", l.click_count);
        }
        
        Message::Raw { .. } => {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // 加载配置文件（如果指定）
    let config_from_file = if let Some(config_path) = &args.config {
        Some(CliConfig::from_file(config_path)?)
    } else {
        None
    };

    // 合并配置（命令行参数 > 配置文件 > 默认值）
    let room_id = if args.room_id != 3151244 {
        // 命令行指定了非默认值
        args.room_id
    } else {
        // 使用配置文件的值或默认值
        config_from_file.as_ref().and_then(|c| c.room_id).unwrap_or(args.room_id)
    };
    
    let cookie = args.cookie.or_else(|| config_from_file.as_ref().and_then(|c| c.cookie.clone()))
        .ok_or("必须提供 Cookie（通过 --cookie 或配置文件）")?;
    
    let message_types = if args.message_types != vec!["user".to_string()] {
        // 命令行指定了非默认值
        Some(args.message_types)
    } else {
        // 使用配置文件的值或默认值
        config_from_file.as_ref().and_then(|c| c.message_types.clone())
    };

    // 解析 Cookie
    let cookie_source = CookieSource::parse(&cookie);
    
    // 统一转换为内联 Cookie 格式 (SESSDATA=xxx;buvid3=yyy) 用于 API 调用和客户端初始化
    let standard_cookie_string = match &cookie_source {
        CookieSource::JsonFile(_) | CookieSource::JsonString(_) => {
            let json_content = cookie_source.get_content()?;
            let json: serde_json::Value = serde_json::from_str(&json_content)?;
            
            let sessdata = json["SESSDATA"]
                .as_str()
                .ok_or("Cookie 中缺少 SESSDATA 字段")?;
            let buvid3 = json["buvid3"].as_str().unwrap_or("");
            
            format!("SESSDATA={};buvid3={}", sessdata, buvid3)
        }
        CookieSource::InlineString(s) => s.clone(),
    };

    // 根据 Cookie 类型创建客户端
    let mut client = match cookie_source {
        CookieSource::JsonFile(path) => {
            // 文件路径：使用 BliveClient::new
            BliveClient::new(room_id, &path)?
        }
        CookieSource::JsonString(_) | CookieSource::InlineString(_) => {
            // JSON 字符串或内联字符串：使用转换后的内联格式初始化客户端
            BliveClient::from_cookie_string(room_id, standard_cookie_string.clone())?
        }
    };
    
    // 获取用户信息（仅在非静默模式下）
    if !args.quiet {
        match get_user_info_from_api(&standard_cookie_string).await {
            Ok((uid, username)) => {
                eprintln!("👤 已登录: {} (UID: {})", username, uid);
            }
            Err(e) => {
                eprintln!("⚠️  无法获取用户信息: {}", e);
            }
        }
    }
    
    let mut stream = client.stream().await?;
    
    // 创建消息过滤器
    let filter = MessageFilter::new(message_types);
    
    // 显示启动信息（仅在非静默模式下）
    if !args.quiet {
        eprintln!("🎮 开始接收弹幕... (按 Ctrl+C 退出)");
    }
    
    while let Some(message) = stream.next().await {
        // 检查是否应该显示该消息
        if filter.should_show(message.cmd()) {
            if args.quiet {
                print_plain_message(&message);
            } else {
                print_colored_message(&message);
            }
        }
    }

    Ok(())
}
