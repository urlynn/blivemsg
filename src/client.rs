use std::path::Path;
use futures_util::stream::{Stream, StreamExt};

use crate::internal;
use crate::message::Message;
use crate::error::Error;

/// B站直播消息客户端
pub struct BliveClient {
    room_id: u64,
    cookie_source: internal::cookie::CookieSource,
    task_handle: Option<tokio::task::JoinHandle<()>>,
}

impl Drop for BliveClient {
    fn drop(&mut self) {
        if let Some(handle) = self.task_handle.take() {
            handle.abort();
        }
    }
}

impl BliveClient {
    /// 创建新的客户端实例
    ///
    /// # 参数
    ///
    /// * `room_id` - 直播间ID
    /// * `cookie_path` - Cookie文件路径（JSON格式）
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use blivemsg::BliveClient;
    ///
    /// let client = BliveClient::new(7734200, "cookies.json")?;
    /// # Ok::<(), blivemsg::Error>(())
    /// ```
    pub fn new(room_id: u64, cookie_path: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(Self {
            room_id,
            cookie_source: internal::cookie::CookieSource::from(cookie_path.as_ref()),
            task_handle: None,
        })
    }
    
    /// 从Cookie字符串创建客户端
    ///
    /// # 参数
    ///
    /// * `room_id` - 直播间ID
    /// * `cookie` - Cookie字符串（内联格式：`SESSDATA=xxx;buvid3=xxx`）
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use blivemsg::BliveClient;
    ///
    /// let client = BliveClient::from_cookie_string(7734200, "SESSDATA=xxx;buvid3=yyy".to_string())?;
    /// # Ok::<(), blivemsg::Error>(())
    /// ```
    pub fn from_cookie_string(room_id: u64, cookie: String) -> Result<Self, Error> {
        Ok(Self {
            room_id,
            cookie_source: internal::cookie::CookieSource::Inline(cookie),
            task_handle: None,
        })
    }
    
    /// 启动客户端并返回消息流（推荐模式）
    ///
    /// 此方法会建立WebSocket连接并返回一个异步流，你可以使用 `StreamExt` 来迭代消息。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use blivemsg::BliveClient;
    /// use futures_util::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), blivemsg::Error> {
    ///     let mut client = BliveClient::new(7734200, "cookies.json")?;
    ///     let mut stream = client.stream().await?;
    ///
    ///     while let Some(message) = stream.next().await {
    ///         match message {
    ///             blivemsg::Message::Danmu(d) => println!("[弹幕] {}: {}", d.username, d.content),
    ///             blivemsg::Message::Gift(g) => println!("[礼物] {} 送了 {}", g.username, g.gift_name),
    ///             _ => {}
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn stream(&mut self) -> Result<impl Stream<Item = Message>, Error> {
        // Ring crypto provider
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            let _ = rustls::crypto::ring::default_provider().install_default();
        });
        let (rx, handle) = internal::ws::start_connection(self.room_id, &self.cookie_source).await?;
        self.task_handle = Some(handle);
        Ok(tokio_stream::wrappers::UnboundedReceiverStream::new(rx))
    }
    
    /// 启动客户端并使用回调处理消息
    ///
    /// 此方法会建立WebSocket连接，并对每条消息调用提供的回调函数。
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use blivemsg::BliveClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), blivemsg::Error> {
    ///     let mut client = BliveClient::new(7734200, "cookies.json")?;
    ///
    ///     client.on_message(|msg| {
    ///         match msg {
    ///             blivemsg::Message::Danmu(d) => println!("[弹幕] {}: {}", d.username, d.content),
    ///             _ => {}
    ///         }
    ///     }).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn on_message<F>(&mut self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(Message) + Send + 'static,
    {
        let mut stream = self.stream().await?;
        
        while let Some(message) = stream.next().await {
            callback(message);
        }
        
        Ok(())
    }
    
    /// 便捷方法：只处理弹幕消息
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use blivemsg::BliveClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), blivemsg::Error> {
    ///     let mut client = BliveClient::new(7734200, "cookies.json")?;
    ///
    ///     client.on_danmu(|danmu| {
    ///         println!("[{}] {}: {}", danmu.medal_level, danmu.username, danmu.content);
    ///     }).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn on_danmu<F>(&mut self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(crate::message::Danmu) + Send + 'static,
    {
        self.on_message(move |msg| {
            if let Message::Danmu(danmu) = msg {
                callback(danmu);
            }
        }).await
    }
    
    /// 便捷方法：只处理礼物消息
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use blivemsg::BliveClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), blivemsg::Error> {
    ///     let mut client = BliveClient::new(7734200, "cookies.json")?;
    ///
    ///     client.on_gift(|gift| {
    ///         println!("{} 送了 {} x{}", gift.username, gift.gift_name, gift.num);
    ///     }).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn on_gift<F>(&mut self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(crate::message::Gift) + Send + 'static,
    {
        self.on_message(move |msg| {
            if let Message::Gift(gift) = msg {
                callback(gift);
            }
        }).await
    }
    
    /// 便捷方法：只处理SuperChat消息
    ///
    /// # 示例
    ///
    /// ```no_run
    /// use blivemsg::BliveClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), blivemsg::Error> {
    ///     let mut client = BliveClient::new(7734200, "cookies.json")?;
    ///
    ///     client.on_super_chat(|sc| {
    ///         println!("[SC ¥{:.2}] {}: {}", sc.price, sc.username, sc.content);
    ///     }).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn on_super_chat<F>(&mut self, mut callback: F) -> Result<(), Error>
    where
        F: FnMut(crate::message::SuperChat) + Send + 'static,
    {
        self.on_message(move |msg| {
            if let Message::SuperChat(sc) = msg {
                callback(sc);
            }
        }).await
    }
}
