use std::time::Duration;

/// Connection settings for the [`crate::Client`].
#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub timeout: Duration,
    pub max_retries: u32,
}

impl ClientConfig {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: None,
            timeout: Duration::from_secs(30),
            max_retries: 2,
        }
    }

    pub fn from_env() -> crate::error::Result<Self> {
        let api_key = std::env::var(crate::API_KEY_ENV).map_err(|_| {
            crate::error::Error::Config(format!(
                "missing {} environment variable",
                crate::API_KEY_ENV
            ))
        })?;
        Ok(Self::new(crate::DEFAULT_BASE_URL).with_api_key(api_key))
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }
}
