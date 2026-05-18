use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use serde_json::json;
use tokio::sync::mpsc;

#[cfg(feature = "http-wreq")]
use wreq::Client;

#[cfg(feature = "http-reqwest")]
use reqwest::Client;

use crate::internal::{http, parser, cookie};
use crate::message::Message;
use crate::error::Error;

/// 构建数据包
fn build_packet(body: Vec<u8>) -> Vec<u8> {
    let packet_len = body.len() as u32 + 16;
    let mut packet = Vec::new();
    packet.extend_from_slice(&packet_len.to_be_bytes());
    packet.extend_from_slice(&16u16.to_be_bytes());
    packet.extend_from_slice(&1u16.to_be_bytes()); // ver = 1 (未压缩)
    packet.extend_from_slice(&7u32.to_be_bytes()); // operation = 7 (AUTH)
    packet.extend_from_slice(&1u32.to_be_bytes()); // seq_id = 1
    packet.extend_from_slice(&body);
    packet
}

/// 构建心跳包
fn build_heartbeat_packet() -> Vec<u8> {
    vec![0, 0, 0, 16, 0, 16, 0, 1, 0, 0, 0, 2, 0, 0, 0, 1]
}

/// 解压缩 brotli 数据
fn decompress_brotli(data: &[u8]) -> Result<Vec<u8>, Error> {
    use std::io::prelude::*;
    let mut decoder = brotli::Decompressor::new(std::io::Cursor::new(data), 4096);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

/// 解压缩 zlib 数据
fn decompress_zlib(data: &[u8]) -> Result<Vec<u8>, Error> {
    use std::io::prelude::*;
    let mut decoder = flate2::read::ZlibDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

/// 解析多个数据包并通过 channel 发送
fn parse_and_send_packets(data: &[u8], tx: &mpsc::UnboundedSender<Message>) {
    let mut offset = 0;
    while offset < data.len() {
        if offset + 16 > data.len() {
            break;
        }
        
        let packet_len = u32::from_be_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]);
        let header_len = u16::from_be_bytes([data[offset+4], data[offset+5]]);
        let ver = u16::from_be_bytes([data[offset+6], data[offset+7]]);
        let op = u32::from_be_bytes([data[offset+8], data[offset+9], data[offset+10], data[offset+11]]);
        
        if packet_len < 16 || header_len < 16 || header_len as u32 > packet_len {
            break;
        }
        
        if packet_len as usize > data.len() - offset {
            break;
        }
        
        let body_start = offset + header_len as usize;
        let body_end = offset + packet_len as usize;
        
        if body_start > body_end || body_end > data.len() {
            break;
        }
        
        let body = &data[body_start..body_end];
        
        if op == 5 {
            // 弹幕消息
            if ver == 0 {
                // 未压缩的JSON
                if let Ok(json_str) = std::str::from_utf8(body)
                    && !json_str.trim().is_empty()
                        && let Ok(json_value) = serde_json::from_str::<serde_json::Value>(json_str)
                            && let Some(cmd) = json_value.get("cmd").and_then(|c| c.as_str()) {
                                // 尝试解析为强类型消息
                                if let Some(message) = parser::parse_message(cmd, &json_value) {
                                    let _ = tx.send(message);
                                } else {
                                    // 未解析的消息包装为 Raw
                                    let _ = tx.send(Message::Raw {
                                        cmd: cmd.to_string(),
                                        data: json_value,
                                    });
                                }
                            }
            } else if ver == 2 {
                // zlib 压缩
                if let Ok(decompressed) = decompress_zlib(body) {
                    parse_and_send_packets(&decompressed, tx);
                }
            } else if ver == 3 {
                // brotli 压缩
                if let Ok(decompressed) = decompress_brotli(body) {
                    parse_and_send_packets(&decompressed, tx);
                }
            }
        }
        
        offset += packet_len as usize;
    }
}

/// 启动 WebSocket 连接并返回消息接收器和取消句柄
pub async fn start_connection(
    room_id: u64,
    cookie_source: &cookie::CookieSource,
) -> Result<(mpsc::UnboundedReceiver<Message>, tokio::task::JoinHandle<()>), Error> {
    let cookies = cookie_source.get_cookie()?;
    
    // 创建 HTTP 客户端
    let client = Client::new();
    
    // 获取用户信息（失败时使用匿名 UID）
    let (_uid, _uname) = match http::get_user_info(&client, &cookies).await {
        Ok(info) => info,
        Err(_) => (0, "Anonymous".to_string()),
    };
    
    // 获取 WBI 密钥
    let wbi_key = http::get_wbi_key(&client, &cookies).await?;
    
    // 获取真实房间 ID
    let real_room_id = http::get_real_room_id(&client, room_id, &cookies).await?;
    
    // 获取弹幕服务器信息
    let (host, port, token) = http::get_danmu_server_info(&client, real_room_id, &cookies, &wbi_key).await?;
    
    // 连接 WebSocket
    let url = format!("wss://{}:{}/sub", host, port);
    let (mut ws_stream, _) = connect_async(url).await?;
    
    // 获取 buvid
    let buvid = http::get_buvid(&client, &cookies).await?;
    
    // 发送认证包
    let mut auth_body = json!({});
    auth_body["uid"] = json!(_uid);
    auth_body["roomid"] = json!(real_room_id);
    auth_body["protover"] = json!(3);
    auth_body["platform"] = json!("web");
    auth_body["type"] = json!(2);
    auth_body["buvid"] = json!(buvid);
    auth_body["key"] = json!(token);

    let body = serde_json::to_vec(&auth_body)?;
    let auth_packet = build_packet(body);
    
    ws_stream.send(WsMessage::Binary(auth_packet.into())).await?;
    
    // 创建消息通道
    let (tx, rx) = mpsc::unbounded_channel();
    
    // 启动消息处理任务
    let handle = tokio::spawn(async move {
        let mut authenticated = false;
        let mut heartbeat_interval = tokio::time::interval(tokio::time::Duration::from_secs(25));
        heartbeat_interval.tick().await;
        
        loop {
            tokio::select! {
                result = ws_stream.next() => {
                    match result {
                        Some(Ok(WsMessage::Binary(data))) => {
                            if data.len() < 16 {
                                continue;
                            }
                            
                            let op = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);
                            
                            match op {
                                8 => {
                                    // 认证成功
                                    authenticated = true;
                                    
                                    // 认证成功后立即发送心跳
                                    let heartbeat_packet = build_heartbeat_packet();
                                    if ws_stream.send(WsMessage::Binary(heartbeat_packet.into())).await.is_err() {
                                        break;
                                    }
                                }
                                5 => {
                                    // 弹幕消息
                                    parse_and_send_packets(&data, &tx);
                                }
                                _ => {}
                            }
                        }
                        Some(Ok(WsMessage::Close(_))) |
                        Some(Err(_)) |
                        None => {
                            break;
                        }
                        Some(Ok(WsMessage::Ping(data))) => {
                            let _ = ws_stream.send(WsMessage::Pong(data)).await;
                        }
                        _ => {}
                    }
                }
                _ = heartbeat_interval.tick(), if authenticated => {
                    // 定期发送心跳（每25秒）
                    let heartbeat_packet = build_heartbeat_packet();
                    if ws_stream.send(WsMessage::Binary(heartbeat_packet.into())).await.is_err() {
                        break;
                    }
                }
            }
        }
    });
    
    Ok((rx, handle))
}
