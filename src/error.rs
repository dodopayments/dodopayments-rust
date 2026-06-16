use thiserror::Error;

/// Convenience alias for results returned by this SDK.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by the SDK client.
#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("failed to (de)serialize JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API returned status {status}: {message}")]
    Api { status: u16, message: String },

    #[error("configuration error: {0}")]
    Config(String),
}
