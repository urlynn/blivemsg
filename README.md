# Bilibili Live Message Client

一个功能简单的 Bilibili 直播间消息 WebSocket 客户端 Rust 库，仅支持实时直播间消息监控、不支持文字转语音（TTS）也不支持浏览器 Cookie 自动检测，提供强类型消息接口和 Stream API。

## ✨ 特性

- **52+ 消息类型** - 完整支持弹幕、礼物、SC、舰长等消息类型
- **体积小巧** - `cli`可执行文件仅 `1.97M(MacOS)` | `2.4M(Windows)` | `2.3M(Linux)`
- **多平台兼容** - 可在[Releases](https://github.com/urlynn/blivemsg/releases)下载预编译`Windows`、`MacOS`和`Linux`版本
- **模块化设计** - 通过 Cargo features 按需启用功能,保持最小依赖
- **HTTP 后端可选** - 支持 reqwest(轻量)和 wreq(浏览器指纹模拟)两种后端切换
- **零拷贝解析** - 高效的 JSON 解析,最小化内存分配

## 🚀 快速开始

---

## 🛠️ CLI 工具

- **从 crates.io 安装**

  ```shell
  cargo install blivemsg --features cli
  ```

- **或下载可执行文件**

  > 访问 [Releases](https://github.com/urlynn/blivemsg/releases) 页面下载预编译可执行文件

### 快速使用

```bash
# 基本用法：指定直播间和 Cookie
blivemsg-cli -r 3151244 -c cookies.json

# 筛选指定消息类型
blivemsg-cli -c cookie.json -t "DANMU_MSG,SEND_GIFT"

# 使用配置文件
blivemsg-cli -C my_config.toml
```

### 命令行参数

```
Usage: blivemsg-cli [OPTIONS]

Options:
  -r, --room-id <ROOM_ID>          直播间 ID [default: 3151244]
  -c, --cookie <COOKIE>            Cookie（JSON文件路径/JSON字符串/内联字符串）
  -t, --message-types <TYPES>      消息类型筛选 [default: USER]
  -q, --quiet                      静默模式（纯文本输出，适合二次处理）
  -C, --config <CONFIG>            从配置文件载入上述参数（TOML格式）
  -h, --help                       Print help
  -V, --version                    Print version
```

#### Cookie 输入方式（自动检测）

```bash
# 1. JSON 文件路径
-c cookies.json

# 2. JSON 字符串
-c '{"SESSDATA":"xxx","buvid3":"xxx"}'

# 3. 内联字符串
-c "SESSDATA=xxx;buvid3=xxx"
```

#### 消息类型筛选

```bash
# 预设类别
-t USER          # 用户消息（默认）
-t SYSTEM        # 系统消息
-t ALL           # 全部消息

# 具体类型（逗号分隔）
-t DANMU_MSG                     # 弹幕
-t DANMU_MSG,SEND_GIFT,SUPER_CHAT_MESSAGE          # 弹幕+礼物+SC
```

**消息类型列表**：完整列表请参考 [MESSAGES.md](MESSAGES.md)

### 配置文件

```
# blivemsg.toml
room_id = 7734200
cookie = "SESSDATA=xxx;buvid3=xxx"
message_types = ["DANMU_MSG", "SEND_GIFT"]
```


**优先级**：命令行参数 > 配置文件 > 默认值

---

### 作为库使用（最小依赖）

#### 添加到项目

```toml
[dependencies]
blivemsg = "0.2.4"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
futures-util = "0.3"
```

**默认配置**: 最小依赖,仅包含核心库功能

**可选功能**(需手动启用):
- `cli` - CLI 命令行工具
- `protobuf-support` - Protobuf 消息解析
- `http-wreq` / `http-reqwest` - HTTP 后端选择

#### 启用可选功能

```toml
[dependencies]
# 启用 Protobuf 支持（用于 INTERACT_WORD_V2 消息）
blivemsg = { version = "0.2.2", features = ["protobuf-support"] }

# HTTP 客户端后端选择（二选一，默认为 http-reqwest）
# 选项 1: reqwest (纯 Rust后端, 编译友好)
blivemsg = { version = "0.2.2", default-features = false, features = ["http-reqwest"] }

# 选项 2: wreq (实验性选项)
blivemsg = { version = "0.2.2", default-features = false, features = ["http-wreq"] }

# 组合多个功能
blivemsg = { version = "0.2.2", features = ["protobuf-support", "http-reqwest"] }
```

#### Stream 模式（推荐）

```rust
use blivemsg::{BliveClient, types::Message};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), blivemsg::Error> {
    let mut client = BliveClient::new(7734200, "cookies.json")?;
    let mut stream = client.stream().await?;
    
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

#### 回调模式

```rust
use blivemsg::BliveClient;

#[tokio::main]
async fn main() -> Result<(), blivemsg::Error> {
    let mut client = BliveClient::new(7734200, "cookies.json")?;
    
    client.on_danmu(|danmu| {
        println!("[{}] {}: {}", danmu.medal_level, danmu.username, danmu.content);
    }).await?;
    
    Ok(())
}
```

## 📦 Features 说明

| Feature | 说明 | 额外依赖 | 适用场景 |
|---------|------|----------|----------|
| `cli` | CLI 工具（**自动包含 protobuf-support**） | clap, toml, prost, bytes, base64 | 需要命令行工具 |
| `protobuf-support` | Protobuf 消息解析 | prost, bytes, base64 | 库用户需要 INTERACT_WORD_V2 消息 |
| `http-wreq` | wreq HTTP 后端 | wreq | 实验性选项; 可能增加稳定性 |
| `http-reqwest` | reqwest HTTP 后端（默认，rustls ring） | reqwest | 跨平台编译兼容 |
| 默认（无） | 核心库功能（使用 http-reqwest + ring） | 无 | 作为库集成到其他项目 |

**注意**: `http-wreq` 和 `http-reqwest` 只能选择一个，不能同时启用。

---

## 📚 文档结构

- **[README.md](README.md)** - 快速开始和 CLI 使用指南（本文件）
- **[API.md](API.md)** - 库 API 详细文档
- **[MESSAGES.md](MESSAGES.md)** - 完整消息类型列表
- **[简单示例](https://github.com/urlynn/PSLinkB/blob/main/example.rs)** - 另一项目中的简单实例

---

## 🏗️ 架构设计

```
src/
├── lib.rs              # 公共 API 导出
├── client.rs           # BliveClient 实现
├── message.rs          # 52 种消息类型定义
├── error.rs            # 统一错误类型
└── internal/           # 私有实现
    ├── ws.rs           # WebSocket 连接
    ├── http.rs         # HTTP API 调用
    ├── parser.rs       # JSON → Message 转换
    └── cookie.rs       # Cookie 管理
```

---

## 📄 许可证

Apache-2.0 OR MIT

## 相关链接

- [GitHub](https://github.com/urlynn/blivemsg)
- [crates.io](https://crates.io/crates/blivemsg)
- [docs.rs](https://docs.rs/blivemsg)

# 更新日志

查看完整更新历史: [CHANGELOG.md](https://github.com/urlynn/blivemsg/blob/main/CHANGELOG.md)

## [0.2.4] - 2026-06-03

### 依赖更新
- reqwest: 0.12 → 0.13（feature `rustls-tls-native-roots` → `rustls-no-provider`，由 rustls ring 提供加密）
- md-5 → md5 0.8.0，签名简化
- 移除 hex
- 移除 wreq-util

## [0.2.3] - 2026-05-19

#### 修复buvid3自动获取

#### 优化部分消息字段
- Gift
- ComboSend
- LikeInfoV3Click
- EntryEffect

#### SuperChat 结构体字段调整
- 移除`start_time`和`end_time`字段

## [0.2.2] - 2026-05-17

### 新增功能

#### HTTP 后端可选配置
引入 Cargo features 支持两种 HTTP 客户端后端,解决跨平台编译问题:

- **http-reqwest** (默认): 使用纯 Rust TLS 后端 rustls
- **http-wreq**: 保留原有浏览器指纹模拟能力

#### WebSocket TLS 后端统一
将 `tokio-tungstenite` 的 TLS 后端从 `native-tls` 切换为 `rustls-tls-native-roots`:
