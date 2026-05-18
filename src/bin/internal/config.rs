use serde::Deserialize;
use std::path::Path;

/// CLI 配置结构（TOML 格式）
#[derive(Debug, Clone, Deserialize)]
pub struct CliConfig {
    /// 直播间ID
    #[allow(dead_code)]
    pub room_id: Option<u64>,
    
    /// Cookie（文件路径、JSON字符串或内联字符串）
    pub cookie: Option<String>,
    
    /// 消息类型筛选列表
    pub message_types: Option<Vec<String>>,
}

impl CliConfig {
    /// 从 TOML 文件加载配置
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: CliConfig = toml::from_str(&content)?;
        Ok(config)
    }
}

/// Cookie 来源类型
#[derive(Debug, Clone)]
pub enum CookieSource {
    /// JSON 文件路径
    JsonFile(String),
    /// JSON 字符串（如 '{"SESSDATA":"xxx","buvid3":"xxx"}'）
    JsonString(String),
    /// 内联字符串（如 "SESSDATA=xxx;buvid3=xxx"）
    InlineString(String),
}

impl CookieSource {
    /// 自动检测 Cookie 输入类型
    pub fn parse(input: &str) -> Self {
        let trimmed = input.trim();
        
        // 1. 以 { 开头 → JSON 字符串
        if trimmed.starts_with('{') {
            CookieSource::JsonString(input.to_string())
        }
        // 2. 包含 = 符号 → 内联字符串
        else if trimmed.contains('=') {
            CookieSource::InlineString(input.to_string())
        }
        // 3. 其他 → 文件路径
        else {
            CookieSource::JsonFile(input.to_string())
        }
    }
    
    /// 获取 Cookie 内容（统一返回 JSON 格式）
    pub fn get_content(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            CookieSource::JsonFile(path) => {
                // 检查文件是否存在
                if !Path::new(path).exists() {
                    return Err(format!("Cookie 文件不存在: {}", path).into());
                }
                Ok(std::fs::read_to_string(path)?)
            }
            CookieSource::JsonString(json) => Ok(json.clone()),
            CookieSource::InlineString(inline) => {
                // 将内联字符串转换为 JSON 格式
                let mut json_obj = serde_json::Map::new();
                
                for part in inline.split(';') {
                    let part = part.trim();
                    if let Some((key, value)) = part.split_once('=') {
                        json_obj.insert(
                            key.trim().to_string(),
                            serde_json::Value::String(value.trim().to_string()),
                        );
                    }
                }
                
                Ok(serde_json::to_string(&json_obj)?)
            }
        }
    }
}
