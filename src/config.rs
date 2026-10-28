use std::time::Duration;

use crate::workload_identity::WorkloadIdentity;

/// Connection settings for the [`crate::Client`].
#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    /// Exchange a workload OIDC token for an access token. Do not set together with `api_key`.
    pub workload_identity: Option<WorkloadIdentity>,
    pub timeout: Duration,
    pub max_retries: u32,
}

impl ClientConfig {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: None,
            workload_identity: None,
            timeout: Duration::from_secs(30),
            max_retries: 2,
        }
    }

    pub fn from_environment(environment: crate::Environment) -> Self {
        Self::new(environment.base_url())
    }

    /// Reads the API key from the environment. If the API key is not set and the identity
    /// provider ID is set, uses workload identity with the detected subject token source.
    pub fn from_env() -> crate::error::Result<Self> {
        let base_url = std::env::var(crate::BASE_URL_ENV)
            .ok()
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| crate::DEFAULT_BASE_URL.to_string());
        if let Ok(api_key) = std::env::var(crate::API_KEY_ENV) {
            return Ok(Self::new(base_url).with_api_key(api_key));
        }
        let source = std::env::var(crate::IDENTITY_PROVIDER_ID_ENV)
            .ok()
            .filter(|value| !value.is_empty())
            .and_then(|_| crate::workload_identity::auto());
        match source {
            Some(source) => {
                Ok(Self::new(base_url).with_workload_identity(WorkloadIdentity::new(source)))
            }
            None => Err(crate::error::Error::Config(format!(
                "missing {} environment variable",
                crate::API_KEY_ENV
            ))),
        }
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn with_workload_identity(mut self, workload_identity: WorkloadIdentity) -> Self {
        self.workload_identity = Some(workload_identity);
        self
    }

    pub fn with_environment(mut self, environment: crate::Environment) -> Self {
        self.base_url = environment.base_url().to_string();
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
