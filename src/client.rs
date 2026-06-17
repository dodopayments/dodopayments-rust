use std::time::Duration;

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

    pub(crate) async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<T> {
        let resp = self.send_with_retry(req).await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(Error::Api {
                status: status.as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(resp.json::<T>().await?)
    }

    pub(crate) async fn handle_empty(&self, req: reqwest::RequestBuilder) -> Result<()> {
        let resp = self.send_with_retry(req).await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(Error::Api {
                status: status.as_u16(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(())
    }

    async fn send_with_retry(&self, builder: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        let max_retries = self.config.max_retries;
        let mut attempt: u32 = 0;
        loop {
            // A non-replayable body cannot be retried, so send the original once.
            let Some(attempt_builder) = builder.try_clone() else {
                return Ok(builder.send().await?);
            };
            let attempt_builder =
                attempt_builder.header("x-stainless-retry-count", attempt.to_string());

            match attempt_builder.send().await {
                Ok(response) => {
                    if attempt < max_retries
                        && should_retry(response.status().as_u16(), response.headers())
                    {
                        tokio::time::sleep(retry_delay(Some(response.headers()), attempt)).await;
                        attempt += 1;
                        continue;
                    }
                    return Ok(response);
                }
                Err(error) => {
                    if attempt < max_retries {
                        tokio::time::sleep(retry_delay(None, attempt)).await;
                        attempt += 1;
                        continue;
                    }
                    return Err(error.into());
                }
            }
        }
    }
}

fn should_retry(status: u16, headers: &reqwest::header::HeaderMap) -> bool {
    // An explicit server hint wins over the status-code heuristic.
    if let Some(hint) = headers
        .get("x-should-retry")
        .and_then(|value| value.to_str().ok())
    {
        if hint == "true" {
            return true;
        }
        if hint == "false" {
            return false;
        }
    }
    status == 408 || status == 409 || status == 429 || status >= 500
}

fn retry_delay(headers: Option<&reqwest::header::HeaderMap>, attempt: u32) -> Duration {
    if let Some(retry_after) = headers.and_then(parse_retry_after) {
        return retry_after;
    }
    let max_delay = Duration::from_secs(8);
    let factor = 1u64.checked_shl(attempt.min(20)).unwrap_or(u64::MAX);
    let base = Duration::from_millis(500u64.saturating_mul(factor)).min(max_delay);
    base.saturating_sub(random_below(base / 4))
}

fn parse_retry_after(headers: &reqwest::header::HeaderMap) -> Option<Duration> {
    if let Some(value) = headers
        .get("retry-after-ms")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<f64>().ok())
        .filter(|value| value.is_finite())
    {
        return Some(Duration::from_secs_f64((value / 1000.0).max(0.0)));
    }
    let value = headers
        .get("retry-after")
        .and_then(|value| value.to_str().ok())?;
    if let Ok(seconds) = value.parse::<f64>() {
        if seconds.is_finite() {
            return Some(Duration::from_secs_f64(seconds.max(0.0)));
        }
    }
    let when = httpdate::parse_http_date(value).ok()?;
    Some(
        when.duration_since(std::time::SystemTime::now())
            .unwrap_or(Duration::ZERO),
    )
}

fn random_below(ceiling: Duration) -> Duration {
    let nanos = ceiling.as_nanos() as u64;
    if nanos == 0 {
        return Duration::ZERO;
    }
    use rand::Rng;
    Duration::from_nanos(rand::thread_rng().gen_range(0..nanos))
}

/// Carries the client and original request shape so a page can fetch its successor.
#[derive(Clone, Debug)]
pub(crate) struct PaginationContext {
    client: Client,
    method: reqwest::Method,
    path: String,
    query: serde_json::Value,
}

impl PaginationContext {
    pub(crate) fn new(
        client: Client,
        method: reqwest::Method,
        path: String,
        query: serde_json::Value,
    ) -> Self {
        Self {
            client,
            method,
            path,
            query,
        }
    }

    pub(crate) fn int_param(&self, name: &str) -> i64 {
        self.query
            .get(name)
            .and_then(|value| {
                value
                    .as_i64()
                    .or_else(|| value.as_str().and_then(|text| text.parse().ok()))
            })
            // Page numbers are 0-indexed: an omitted page param is the first page (0), so
            // get_next_page advances to 1. Defaulting to 1 here would skip the second page.
            .unwrap_or(0)
    }

    pub(crate) fn with_param(&self, name: &str, value: serde_json::Value) -> Self {
        let mut next = self.clone();
        match next.query.as_object_mut() {
            Some(map) => {
                map.insert(name.to_string(), value);
            }
            None => {
                let mut map = serde_json::Map::new();
                map.insert(name.to_string(), value);
                next.query = serde_json::Value::Object(map);
            }
        }
        next
    }

    pub(crate) async fn fetch<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        let request = self
            .client
            .request(self.method.clone(), &self.path)
            .query(&self.query);
        self.client.handle_response::<T>(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::{HeaderMap, HeaderValue};

    fn headers(pairs: &[(&'static str, &'static str)]) -> HeaderMap {
        let mut map = HeaderMap::new();
        for (name, value) in pairs {
            map.insert(*name, HeaderValue::from_static(value));
        }
        map
    }

    #[test]
    fn retries_transient_status_codes() {
        let empty = HeaderMap::new();
        assert!(should_retry(408, &empty));
        assert!(should_retry(409, &empty));
        assert!(should_retry(429, &empty));
        assert!(should_retry(500, &empty));
        assert!(should_retry(503, &empty));
    }

    #[test]
    fn does_not_retry_client_errors() {
        let empty = HeaderMap::new();
        assert!(!should_retry(200, &empty));
        assert!(!should_retry(400, &empty));
        assert!(!should_retry(404, &empty));
        assert!(!should_retry(422, &empty));
    }

    #[test]
    fn should_retry_header_overrides_status() {
        assert!(should_retry(200, &headers(&[("x-should-retry", "true")])));
        assert!(!should_retry(500, &headers(&[("x-should-retry", "false")])));
    }

    #[test]
    fn backoff_grows_and_caps() {
        let first = retry_delay(None, 0);
        assert!(first <= Duration::from_millis(500) && first >= Duration::from_millis(375));

        let second = retry_delay(None, 1);
        assert!(second <= Duration::from_millis(1000) && second >= Duration::from_millis(750));

        let capped = retry_delay(None, 12);
        assert!(capped <= Duration::from_secs(8) && capped >= Duration::from_secs(6));
    }

    #[test]
    fn retry_after_ms_takes_priority_over_seconds() {
        let map = headers(&[("retry-after", "2"), ("retry-after-ms", "500")]);
        assert_eq!(retry_delay(Some(&map), 0), Duration::from_millis(500));
    }

    #[test]
    fn retry_after_seconds_is_honored() {
        let map = headers(&[("retry-after", "3")]);
        assert_eq!(parse_retry_after(&map), Some(Duration::from_secs(3)));
    }

    #[test]
    fn missing_page_number_defaults_to_zero() {
        let client = Client::new(ClientConfig::new("https://example.invalid")).unwrap();
        let ctx = PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            "/items".to_string(),
            serde_json::json!({ "page_size": 10 }),
        );
        assert_eq!(ctx.int_param("page_number"), 0);

        let ctx = PaginationContext::new(
            client,
            reqwest::Method::GET,
            "/items".to_string(),
            serde_json::json!({ "page_number": 2 }),
        );
        assert_eq!(ctx.int_param("page_number"), 2);
    }
}
