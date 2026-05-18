# blivemsg

[![Crates.io](https://img.shields.io/crates/v/blivemsg.svg)](https://crates.io/crates/blivemsg)
[![Documentation](https://docs.rs/blivemsg/badge.svg)](https://docs.rs/blivemsg)
[![License](https://img.shields.io/crates/l/blivemsg.svg)](https://github.com/urlynn/blivemsg/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/urlynn/blivemsg?style=social)](https://github.com/urlynn/blivemsg)

**Bilibili Live Message Library** - 轻量高效的B站直播消息 Rust 库

## 🚀 快速开始

### 基础示例

#### Stream 模式（推荐）
```rust,no_run
use blivemsg::BliveClient;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), blivemsg::Error> {
    // 创建客户端（从 Cookie 文件加载）
    let mut client = BliveClient::new(7734200, "cookies.json")?;
    
    // 获取消息流
    let mut stream = client.stream().await?;
    
    // 处理消息
    while let Some(message) = stream.next().await {
        match message {
            Message::Danmu(d) => println!("[{}] {}: {}", d.medal_level, d.username, d.content),
            Message::Gift(g) => println!("{} 送了 {} x{}", g.username, g.gift_name, g.num),
            Message::SuperChat(sc) => println!("[SC] ¥{:.2}", sc.price),
            _ => {}
        }
    }
    
    Ok(())
}
```

## ✨ 核心特性

- **52+ 消息类型** - 完整支持弹幕、礼物、SC、舰长等所有消息
- **模块化设计** - 通过 Cargo features 按需启用功能,保持最小依赖
- **HTTP 后端可选** - 支持 reqwest(轻量)和 wreq(浏览器指纹模拟)两种后端切换
- **零拷贝解析** - 高效的 JSON 解析,最小化内存分配

## 📖 文档

- **[API 文档](https://github.com/urlynn/blivemsg/blob/main/API.md)** - 完整的 API 参考
- **[GitHub](https://github.com/urlynn/blivemsg)** - 源码仓库、Issue 反馈
- **[消息列表](https://github.com/urlynn/blivemsg/blob/main/MESSAGES.md)** - 所有支持的消息类型
- **[简单示例](https://github.com/urlynn/PSLinkB/blob/main/src/actors/irc_client_actor.rs)** - 另一项目中的一个简单示例

## 🔧 高级用法

### 使用回调模式

```rust,no_run
use blivemsg::BliveClient;

#[tokio::main]
async fn main() -> Result<(), blivemsg::Error> {
    let mut client = BliveClient::new(7734200, "cookies.json")?;
    
    // 只处理弹幕
    client.on_danmu(|danmu| {
        println!("[{}] {}: {}", danmu.medal_level, danmu.username, danmu.content);
    }).await?;
    
    Ok(())
}
```

### 便捷方法

```rust,no_run
use blivemsg::BliveClient;

#[tokio::main]
async fn main() -> Result<(), blivemsg::Error> {
    let mut client = BliveClient::new(7734200, "cookies.json")?;
    
    // 只处理礼物
    client.on_gift(|gift| {
        println!("{} 送了 {} x{}", gift.username, gift.gift_name, gift.num);
    }).await?;
    
    Ok(())
}
```

## 📦 Cookie 配置

Cookie 包含以下两个字段:
- `SESSDATA` (必须)
- `buvid3` (可选)

支持三种输入格式:
1. JSON 对象: `{"SESSDATA":"xxx","buvid3":"yyy"}`
2. 内联字符串: `"SESSDATA=xxx;buvid3=yyy"`
3. JSON 文件路径

## 🛠️ Features

- **default**: 最小依赖，仅库功能（使用 http-reqwest）
- **cli**: 启用 CLI 工具（需要 clap、toml）
- **protobuf-support**: 启用 Protobuf 消息支持
- **http-reqwest**: 使用 reqwest HTTP 后端（默认，纯 Rust）
- **http-wreq**: 使用 wreq HTTP 后端（浏览器指纹模拟）

**注意**: `http-wreq` 和 `http-reqwest` 只能选择一个。

## 📄 许可证

MIT

## 🔗 相关链接

- [GitHub](https://github.com/urlynn/blivemsg)
- [crates.io](https://crates.io/crates/blivemsg)
- [docs.rs](https://docs.rs/blivemsg)
