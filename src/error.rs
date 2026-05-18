#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cookie error: {0}")]
    CookieError(String),
    
    #[cfg(feature = "http-wreq")]
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] wreq::Error),
    
    #[cfg(feature = "http-reqwest")]
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("WebSocket connection failed: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
    
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Room ID invalid: {0}")]
    InvalidRoomId(u64),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Authentication failed")]
    AuthenticationFailed,
}
