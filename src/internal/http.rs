use std::collections::HashMap;
use serde_json::Value;

#[cfg(feature = "http-wreq")]
use wreq::Client;

#[cfg(feature = "http-reqwest")]
use reqwest::Client;

use crate::error::Error;

// WBI密钥索引表
const WBI_KEY_INDEX_TABLE: [usize; 32] = [
    46, 47, 18, 2, 53, 8, 23, 32, 15, 50, 10, 31, 58, 3, 45, 35,
    27, 43, 5, 49, 33, 9, 42, 19, 29, 28, 14, 39, 12, 38, 41, 13
];

/// 获取当前用户的 UID 和用户名
pub async fn get_user_info(client: &Client, cookies: &str) -> Result<(u64, String), Error> {
    let url = "https://api.bilibili.com/x/web-interface/nav";
    
    let resp = client.get(url)
        .header("Cookie", cookies)
        .send()
        .await?;
    
    let text = resp.text().await?;
    let json: Value = serde_json::from_str(&text)?;
    
    let uid = json["data"]["mid"]
        .as_u64()
        .ok_or_else(|| Error::AuthenticationFailed)?;
    
    let uname = json["data"]["uname"]
        .as_str()
        .ok_or_else(|| Error::AuthenticationFailed)?
        .to_string();
    
    Ok((uid, uname))
}

/// 获取 WBI 密钥
pub async fn get_wbi_key(client: &Client, cookies: &str) -> Result<String, Error> {
    let url = "https://api.bilibili.com/x/web-interface/nav";
    
    let resp = client.get(url)
        .header("Cookie", cookies)
        .send()
        .await?;
    
    let text = resp.text().await?;
    let json: Value = serde_json::from_str(&text)?;
    let wbi_img = json["data"]["wbi_img"]
        .as_object()
        .ok_or(Error::AuthenticationFailed)?;
    
    let img_url = wbi_img["img_url"].as_str().ok_or(Error::AuthenticationFailed)?;
    let sub_url = wbi_img["sub_url"].as_str().ok_or(Error::AuthenticationFailed)?;
    
    let img_key = img_url.rsplit('/').next().unwrap_or("").split('.').next().unwrap_or("");
    let sub_key = sub_url.rsplit('/').next().unwrap_or("").split('.').next().unwrap_or("");
    
    let shuffled_key = format!("{}{}", img_key, sub_key);
    let mut wbi_key = String::new();
    for &index in &WBI_KEY_INDEX_TABLE {
        if index < shuffled_key.len() {
            wbi_key.push(shuffled_key.chars().nth(index).unwrap());
        }
    }
    Ok(wbi_key)
}

/// 获取真实房间 ID
pub async fn get_real_room_id(client: &Client, room_id: u64, cookies: &str) -> Result<u64, Error> {
    let url = format!("https://api.live.bilibili.com/room/v1/Room/get_info?room_id={}", room_id);
    
    let resp = client.get(&url)
        .header("Cookie", cookies)
        .send()
        .await?;
    
    let text = resp.text().await?;
    let json: Value = serde_json::from_str(&text)?;
    let real_room_id = json["data"]["room_id"]
        .as_u64()
        .ok_or(Error::InvalidRoomId(room_id))?;
    Ok(real_room_id)
}

/// 获取弹幕服务器信息（使用 WBI 签名）
pub async fn get_danmu_server_info(
    client: &Client, 
    real_room_id: u64, 
    cookies: &str, 
    wbi_key: &str
) -> Result<(String, u16, String), Error> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), real_room_id.to_string());
    params.insert("type".to_string(), "0".to_string());
    
    let signed_params = add_wbi_sign(params, wbi_key);
    
    let url = "https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo";
    
    let resp = client.get(url)
        .header("Cookie", cookies)
        .header("Referer", "https://live.bilibili.com/")
        .query(&signed_params)
        .send()
        .await?;
    
    let text = resp.text().await?;
    let json: Value = serde_json::from_str(&text)?;
    
    let token = json["data"]["token"]
        .as_str()
        .ok_or(Error::AuthenticationFailed)?
        .to_string();
    
    let host_list = json["data"]["host_list"]
        .as_array()
        .ok_or(Error::AuthenticationFailed)?;
    
    if host_list.is_empty() {
        return Err(Error::AuthenticationFailed);
    }
    
    let host = host_list[0]["host"]
        .as_str()
        .ok_or(Error::AuthenticationFailed)?
        .to_string();
    
    let port = host_list[0]["wss_port"]
        .as_u64()
        .ok_or(Error::AuthenticationFailed)? as u16;
    
    Ok((host, port, token))
}

/// 获取 buvid
pub async fn get_buvid(client: &Client, cookies: &str) -> Result<String, Error> {
    // 首先尝试从Cookie中提取buvid3
    if let Some(start) = cookies.find("buvid3=") {
        let buvid_start = start + 7;
        let buvid_end = cookies[buvid_start..].find(';').unwrap_or(cookies.len() - buvid_start);
        let buvid = cookies[buvid_start..buvid_start + buvid_end].to_string();
        if !buvid.is_empty() {
            return Ok(buvid);
        }
    }
    
    // 如果Cookie中没有buvid3，尝试从API获取
    #[derive(serde::Deserialize)]
    struct GetBuvidResponse {
        code: i32,
        data: BuvidData,
    }
    
    #[derive(serde::Deserialize)]
    struct BuvidData {
        buvid: String,
    }
    
    let resp = client.get("https://api.bilibili.com/x/web-frontend/getbuvid")
        .send()
        .await?;
    
    let buvid_resp: GetBuvidResponse = resp.json().await?;
    
    if buvid_resp.code == 0 && !buvid_resp.data.buvid.is_empty() {
        return Ok(buvid_resp.data.buvid);
    }
    
    Err(Error::CookieError("无法获取 buvid3".to_string()))
}

/// 添加 WBI 签名
fn add_wbi_sign(mut params: HashMap<String, String>, wbi_key: &str) -> HashMap<String, String> {
    if wbi_key.is_empty() {
        return params;
    }
    
    use md5::{Md5, Digest};
    
    let wts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    
    params.insert("wts".to_string(), wts.clone());
    
    // 按key字典序排序
    let mut sorted_keys: Vec<String> = params.keys().cloned().collect();
    sorted_keys.sort();
    
    // 构建要签名的字符串
    let mut str_to_sign = String::new();
    for key in sorted_keys {
        let value = params.get(&key).unwrap();
        // 过滤特殊字符
        let filtered_value: String = value.chars()
            .filter(|c| !matches!(c, '!' | '*' | '\'' | '(' | ')'))
            .collect();
        if !str_to_sign.is_empty() {
            str_to_sign.push('&');
        }
        str_to_sign.push_str(&format!("{}={}", key, filtered_value));
    }
    str_to_sign.push_str(wbi_key);
    
    // MD5签名
    let mut hasher = Md5::new();
    hasher.update(str_to_sign.as_bytes());
    let result = hasher.finalize();
    let w_rid = hex::encode(result);
    params.insert("w_rid".to_string(), w_rid);
    params
}
