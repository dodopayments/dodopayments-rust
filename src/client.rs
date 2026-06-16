use crate::config::ClientConfig;
use crate::error::{Error, Result};

/// The top-level API client. Resource methods are generated onto this type.
#[derive(Clone, Debug)]
pub struct Client {
    config: ClientConfig,
    http: reqwest::Client,
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let http = reqwest::Client::builder().timeout(config.timeout).build()?;
        Ok(Self { config, http })
    }

    pub fn from_env() -> Result<Self> {
        Self::new(ClientConfig::from_env()?)
    }

    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    /// Low-level escape hatch: build a request to an arbitrary path with auth applied.
    pub fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!(
            "{}/{}",
            self.config.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        );
        let req = self.http.request(method, url);
        match &self.config.api_key {
            Some(api_key) => req.bearer_auth(api_key),
            None => req,
        }
    }
}

pub(crate) async fn handle_response<T: serde::de::DeserializeOwned>(
    req: reqwest::RequestBuilder,
) -> Result<T> {
    let resp = req.send().await?;
    let status = resp.status();
    if !status.is_success() {
        return Err(Error::Api {
            status: status.as_u16(),
            message: resp.text().await.unwrap_or_default(),
        });
    }
    Ok(resp.json::<T>().await?)
}

pub(crate) async fn handle_empty(req: reqwest::RequestBuilder) -> Result<()> {
    let resp = req.send().await?;
    let status = resp.status();
    if !status.is_success() {
        return Err(Error::Api {
            status: status.as_u16(),
            message: resp.text().await.unwrap_or_default(),
        });
    }
    Ok(())
}
