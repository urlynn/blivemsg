use std::path::Path;
use serde_json::Value;
use crate::error::Error;

/// Cookie 来源抽象
#[derive(Debug, Clone)]
pub enum CookieSource {
    /// 从文件读取
    File(std::path::PathBuf),
    /// 直接使用字符串
    Inline(String),
}

impl CookieSource {
    /// 获取 Cookie 字符串（只使用 SESSDATA 和 buvid3）
    pub fn get_cookie(&self) -> Result<String, Error> {
        match self {
            CookieSource::File(path) => {
                if !path.exists() {
                    return Err(Error::CookieError(format!("Cookie file not found: {}", path.display())));
                }
                
                // 读取 JSON 文件并提取 SESSDATA 和 buvid3
                let content = std::fs::read_to_string(path)?;
                let json: Value = serde_json::from_str(&content)?;
                
                let sessdata = json["SESSDATA"]
                    .as_str()
                    .ok_or_else(|| Error::CookieError("Missing SESSDATA in cookie file".to_string()))?;
                let buvid3 = json["buvid3"]
                    .as_str()
                    .unwrap_or("");
                
                Ok(format!("SESSDATA={}; buvid3={}", sessdata, buvid3))
            }
            CookieSource::Inline(cookie) => Ok(cookie.clone()),
        }
    }
}

impl From<&str> for CookieSource {
    fn from(s: &str) -> Self {
        // 判断是文件路径还是直接的 Cookie 字符串
        if s.contains("SESSDATA=") || s.contains("buvid3=") {
            CookieSource::Inline(s.to_string())
        } else {
            CookieSource::File(std::path::PathBuf::from(s))
        }
    }
}

impl From<String> for CookieSource {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl From<&Path> for CookieSource {
    fn from(p: &Path) -> Self {
        CookieSource::File(p.to_path_buf())
    }
}
