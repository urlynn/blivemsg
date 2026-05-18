# 更新日志

## [0.2.3] - 2026-05-19

#### 修复buvid3自动获取
- buvid3 现为可选字段

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
引入 Cargo features 支持两种 HTTP 客户端后端, 针对跨平台编译问题:

- **http-reqwest** (默认): 使用纯 Rust TLS 后端 rustls
  - 无需 C 编译器, 编译简单快速
  - 跨平台友好
  - 二进制体积更小
  - 依赖更少
  
- **http-wreq**: 保留原有浏览器指纹模拟能力
  - 需要 C 编译器和工具链
  - 编译时间较长
  - 二进制体积较大
  - 浏览器指纹模拟更完善

#### WebSocket TLS 后端统一
将 `tokio-tungstenite` 的 TLS 后端从 `native-tls` 切换为 `rustls-tls-native-roots`:

- **Before**: `tokio-tungstenite = { version = "0.29.0", features = ["native-tls"] }`
- **Now**: `tokio-tungstenite = { version = "0.29.0", features = ["rustls-tls-native-roots"] }`

**改进点**:
- 与默认的 http-reqwest 后端保持一致,统一使用 rustls-native-roots
- 避免 native-tls 在不同平台的 OpenSSL/libressl 依赖问题
- 减少系统级依赖, 提升跨平台编译成功率
- rustls-native-roots 自动使用系统证书存储

### 技术改进

#### 模块化设计优化
- 通过 `#[cfg(feature = "...")]` 条件编译实现后端隔离
- 所有 HTTP 相关模块(http.rs, ws.rs, cli.rs)均支持双后端

### 依赖调整
- 将 `wreq` 从必需依赖改为可选依赖
- 新增 `reqwest` 作为默认 HTTP 后端
- `tokio-tungstenite` TLS 后端从 `native-tls` 切换为 `rustls-tls-native-roots`

### 迁移指南

#### 对于库用户
```toml
# 保持默认行为(推荐)
blivemsg = "0.2.2"

# 如需使用 wreq 后端
blivemsg = { version = "0.2.2", default-features = false, features = ["http-wreq"] }
```

#### 对于 CLI 用户
```bash
# 默认使用 reqwest 后端
cargo install blivemsg --features cli

# 如需使用 wreq 后端
cargo install blivemsg --features "cli,http-wreq" --no-default-features
```

### 破坏性变更
无。
