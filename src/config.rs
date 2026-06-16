use std::time::Duration;

/// Connection settings for the [`crate::Client`].
#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub timeout: Duration,
}

impl ClientConfig {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: None,
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}
