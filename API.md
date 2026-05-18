# blivemsg API 文档

## 核心类型

### BliveClient

主要客户端结构，提供 Stream 和回调两种模式。

#### 创建客户端

```rust
use blivemsg::BliveClient;

// 从 Cookie 文件创建
let client = BliveClient::new(7734200, "cookies.json")?;

// 从 Cookie 字符串创建
let client = BliveClient::from_cookie_string(7734200, "SESSDATA=xxx;buvid3=xxx")?;
```

#### Stream 模式（推荐）

```rust
use blivemsg::{BliveClient, Message};
use futures_util::StreamExt;

let client = BliveClient::new(7734200, "cookies.json")?;
let mut stream = client.stream().await?;

while let Some(message) = stream.next().await {
    match message {
        Message::Danmu(d) => println!("{}", d.content),
        Message::Gift(g) => println!("{} x{}", g.gift_name, g.num),
        _ => {}
    }
}
```

#### 回调模式

```rust
use blivemsg::BliveClient;

let mut client = BliveClient::new(7734200, "cookies.json")?;

client.on_danmu(|msg| {
    println!("[弹幕] {}: {}", msg.username, msg.content);
});

client.on_gift(|msg| {
    println!("[礼物] {} 送了 {} x{}", msg.username, msg.gift_name, g.num);
});

client.start().await?;
```

### Message 枚举

所有消息类型的枚举，共 52 种。

#### 用户消息（11种）

- `Danmu` - 弹幕
- `Gift` - 礼物
- `SuperChat` - 醒目留言（SC）
- `GuardBuy` - 开通舰长/提督/总督
- `WelcomeGuard` - 欢迎总督/提督
- `ComboSend` - 礼物连击
- `UserToastMsg` / `UserToastMsgV2` - 用户 Toast
- `LikeInfoV3Click` - 点赞
- `EntryEffect` - 入场特效
- `InteractWordV2` - 进入直播间（需 `protobuf-support` feature）

#### 系统消息（38种）

完整列表请参考 [MESSAGES.md](MESSAGES.md)

### Error 枚举

统一错误类型：

```rust
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    WebSocket(tungstenite::Error),
    CookieFileNotFound(std::path::PathBuf),
    AuthenticationFailed,
    InvalidRoomId,
    HttpError(String),
    ParseError(String),
}
```

## 高级用法

### 消息过滤

```rust
use blivemsg::{BliveClient, Message};
use futures_util::StreamExt;

let client = BliveClient::new(7734200, "cookies.json")?;
let stream = client.stream().await?;

// 只接收弹幕和礼物
let filtered = stream.filter(|msg| {
    matches!(msg, Message::Danmu(_) | Message::Gift(_))
});
```

### 自定义处理

```rust
use blivemsg::{BliveClient, Message};
use futures_util::StreamExt;

let client = BliveClient::new(7734200, "cookies.json")?;
let stream = client.stream().await?;

// 使用 map 转换消息
let processed = stream.map(|msg| {
    match msg {
        Message::Danmu(d) => format!("{}: {}", d.username, d.content),
        Message::Gift(g) => format!("{} x{}", g.gift_name, g.num),
        _ => String::new(),
    }
});
```

## Cookie 格式

```json
{
  "SESSDATA": "your_sessdata_here",
  "buvid3": "your_buvid3_here"
}
```

## Feature Flags

### protobuf-support

启用后支持 `INTERACT_WORD_V2` 消息类型：

```toml
[dependencies]
blivemsg = { version = "0.2.2", features = ["protobuf-support"] }
```

### HTTP 后端选择

默认使用 `http-reqwest`（纯 Rust,跨平台友好）。如需使用 wreq（浏览器指纹模拟）：

```toml
[dependencies]
blivemsg = { version = "0.2.2", default-features = false, features = ["http-wreq"] }
```

**注意**: `http-wreq` 和 `http-reqwest` 只能选择一个。

## 完整示例

```rust
use blivemsg::{BliveClient, Message};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), blivemsg::Error> {
    let client = BliveClient::new(7734200, "cookies.json")?;
    let mut stream = client.stream().await?;
    
    while let Some(message) = stream.next().await {
        match message {
            Message::Danmu(d) => {
                println!("[{}] {}: {}", 
                    d.medal_name.unwrap_or_else(|| "无".to_string()),
                    d.username, 
                    d.content
                );
            }
            Message::Gift(g) => {
                println!("[礼物] {} 送出 {} x{}", 
                    g.username, 
                    g.gift_name, 
                    g.num
                );
            }
            Message::SuperChat(sc) => {
                println!("[SC] ¥{:.2} {}", sc.price, sc.username);
            }
            _ => {}
        }
    }
    
    Ok(())
}
```
