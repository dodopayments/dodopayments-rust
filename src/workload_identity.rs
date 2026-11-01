//! Workload identity federation.
//!
//! The client exchanges a short-lived OIDC token from the platform that runs your code
//! (GitHub Actions, GitLab CI, Terraform Cloud, GCP, Vercel, Kubernetes, AWS) for an access
//! token, and refreshes it before it expires. The client never logs either token.

use std::fmt;
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use futures::future::BoxFuture;

use crate::client::Client;
use crate::error::{Error, Result};

const GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:token-exchange";
const SUBJECT_TOKEN_TYPE: &str = "urn:ietf:params:oauth:token-type:jwt";
/// The client gets a new access token when this much time or less remains.
const REFRESH_MARGIN: Duration = Duration::from_secs(300);
const KUBERNETES_TOKEN_PATH: &str = "/var/run/secrets/tokens/dodo-token";
const GCP_IDENTITY_URL: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/identity";
const GITHUB_REQUEST_URL_ENV: &str = "ACTIONS_ID_TOKEN_REQUEST_URL";
const GITHUB_REQUEST_TOKEN_ENV: &str = "ACTIONS_ID_TOKEN_REQUEST_TOKEN";
const TFC_TOKEN_ENV: &str = "TFC_WORKLOAD_IDENTITY_TOKEN";
const TFC_DODO_TOKEN_ENV: &str = "TFC_WORKLOAD_IDENTITY_TOKEN_DODO";
const VERCEL_TOKEN_ENV: &str = "VERCEL_OIDC_TOKEN";
const VERCEL_TOKEN_FILE_ENV: &str = "VERCEL_OIDC_TOKEN_FILE";
const VERCEL_EXCHANGE_URL: &str = "https://oidc.vercel.com/~token";
const VERCEL_EXCHANGE_TIMEOUT: Duration = Duration::from_secs(10);

/// Workload identity settings for [`crate::ClientConfig::with_workload_identity`].
#[derive(Clone, Debug)]
pub struct WorkloadIdentity {
    identity_provider_id: Option<String>,
    provider: SubjectTokenSource,
}

impl WorkloadIdentity {
    /// Uses `provider` for the subject token. The identity provider ID comes from the
    /// `DODO_PAYMENTS_IDENTITY_PROVIDER_ID` environment variable unless you set it.
    pub fn new(provider: SubjectTokenSource) -> Self {
        Self {
            identity_provider_id: None,
            provider,
        }
    }

    pub fn with_identity_provider_id(mut self, identity_provider_id: impl Into<String>) -> Self {
        self.identity_provider_id = Some(identity_provider_id.into());
        self
    }
}

/// The data that a subject token source gets for each exchange.
#[derive(Clone)]
pub struct SubjectTokenContext {
    base_url: String,
    http: reqwest::Client,
}

impl SubjectTokenContext {
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// The HTTP client of the SDK client, with its timeout.
    pub fn http_client(&self) -> &reqwest::Client {
        &self.http
    }

    /// The origin of the base URL, for example `https://live.dodopayments.com`.
    pub fn default_audience(&self) -> String {
        reqwest::Url::parse(&self.base_url)
            .map(|url| url.origin().ascii_serialization())
            .unwrap_or_else(|_| self.base_url.trim_end_matches('/').to_string())
    }
}

/// Gives the subject token (a JWT) for one exchange.
pub trait SubjectTokenProvider: Send + Sync {
    fn subject_token(&self, context: SubjectTokenContext) -> BoxFuture<'_, Result<String>>;
}

struct FnProvider<F>(F);

impl<F, Fut> SubjectTokenProvider for FnProvider<F>
where
    F: Fn(SubjectTokenContext) -> Fut + Send + Sync,
    Fut: Future<Output = Result<String>> + Send + 'static,
{
    fn subject_token(&self, context: SubjectTokenContext) -> BoxFuture<'_, Result<String>> {
        Box::pin((self.0)(context))
    }
}

/// A shared subject token source. Make one with the functions in this module.
#[derive(Clone)]
pub struct SubjectTokenSource {
    kind: &'static str,
    provider: Arc<dyn SubjectTokenProvider>,
}

impl SubjectTokenSource {
    pub fn new(provider: impl SubjectTokenProvider + 'static) -> Self {
        Self {
            kind: "custom",
            provider: Arc::new(provider),
        }
    }

    /// Calls `f` for each exchange.
    pub fn from_fn<F, Fut>(f: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<String>> + Send + 'static,
    {
        Self::with_context("custom", move |_| f())
    }

    fn with_context<F, Fut>(kind: &'static str, f: F) -> Self
    where
        F: Fn(SubjectTokenContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<String>> + Send + 'static,
    {
        Self {
            kind,
            provider: Arc::new(FnProvider(f)),
        }
    }

    async fn subject_token(&self, context: SubjectTokenContext) -> Result<String> {
        self.provider.subject_token(context).await
    }
}

impl fmt::Debug for SubjectTokenSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SubjectTokenSource")
            .field("kind", &self.kind)
            .finish()
    }
}

/// Requests a token from the GitHub Actions OIDC endpoint for each exchange.
/// The default audience is the origin of the client base URL.
pub fn github_actions(audience: Option<&str>) -> SubjectTokenSource {
    let audience = audience.map(str::to_owned);
    SubjectTokenSource::with_context("github_actions", move |context| {
        let audience = audience.clone();
        async move {
            let request_url = required_env(GITHUB_REQUEST_URL_ENV)?;
            let request_token = required_env(GITHUB_REQUEST_TOKEN_ENV)?;
            let mut url = reqwest::Url::parse(&request_url).map_err(|error| {
                Error::Config(format!(
                    "{GITHUB_REQUEST_URL_ENV} is not a valid URL: {error}"
                ))
            })?;
            let audience = audience.unwrap_or_else(|| context.default_audience());
            url.query_pairs_mut().append_pair("audience", &audience);
            let response = context
                .http
                .get(url)
                .header(
                    reqwest::header::AUTHORIZATION,
                    format!("bearer {request_token}"),
                )
                .send()
                .await?;
            let body: GithubIdToken = success(response).await?.json().await?;
            non_empty(body.value, "the GitHub Actions OIDC token")
        }
    })
}

/// Reads the GitLab CI `id_tokens` value from `env_var` (default `DODO_PAYMENTS_ID_TOKEN`).
pub fn gitlab(env_var: Option<&str>) -> SubjectTokenSource {
    let env_var = env_var.unwrap_or(crate::ID_TOKEN_ENV).to_owned();
    SubjectTokenSource::with_context("gitlab", move |_| {
        let value = required_env(&env_var);
        async move { value }
    })
}

/// Reads `TFC_WORKLOAD_IDENTITY_TOKEN`, or `TFC_WORKLOAD_IDENTITY_TOKEN_DODO`.
pub fn terraform_cloud() -> SubjectTokenSource {
    SubjectTokenSource::with_context("terraform_cloud", |_| {
        let value = env_value(TFC_TOKEN_ENV)
            .or_else(|| env_value(TFC_DODO_TOKEN_ENV))
            .ok_or_else(|| {
                Error::Config(format!(
                    "missing {TFC_TOKEN_ENV} or {TFC_DODO_TOKEN_ENV} environment variable"
                ))
            });
        async move { value }
    })
}

/// Requests an identity token from the GCP metadata server for each exchange.
/// The default audience is the origin of the client base URL.
pub fn gcp(audience: Option<&str>) -> SubjectTokenSource {
    let audience = audience.map(str::to_owned);
    SubjectTokenSource::with_context("gcp", move |context| {
        let audience = audience.clone();
        async move {
            let audience = audience.unwrap_or_else(|| context.default_audience());
            let response = context
                .http
                .get(GCP_IDENTITY_URL)
                .query(&[("audience", audience.as_str()), ("format", "full")])
                .header("Metadata-Flavor", "Google")
                .send()
                .await?;
            let token = success(response).await?.text().await?;
            non_empty(token, "the GCP identity token")
        }
    })
}

/// Exchanges the Vercel OIDC token for a token with the Dodo Payments audience, for each
/// exchange. Reads the token from the file in `VERCEL_OIDC_TOKEN_FILE` or from
/// `VERCEL_OIDC_TOKEN`. The audience is the origin of the client base URL.
pub fn vercel() -> SubjectTokenSource {
    vercel_source(None, VERCEL_EXCHANGE_URL.to_owned())
}

/// Is the same as [`vercel`], but uses `audience` when you supply it.
pub fn vercel_with_audience(audience: Option<&str>) -> SubjectTokenSource {
    vercel_source(audience.map(str::to_owned), VERCEL_EXCHANGE_URL.to_owned())
}

fn vercel_source(audience: Option<String>, exchange_url: String) -> SubjectTokenSource {
    SubjectTokenSource::with_context("vercel", move |context| {
        let audience = audience.clone();
        let exchange_url = exchange_url.clone();
        let base_token = vercel_base_token();
        async move {
            let base_token = base_token?;
            let audience = audience.unwrap_or_else(|| context.default_audience());
            let response = context
                .http
                .post(exchange_url)
                .timeout(VERCEL_EXCHANGE_TIMEOUT)
                .header(reqwest::header::ACCEPT, "application/json")
                .json(&VercelExchangeRequest {
                    token: &base_token,
                    aud: &audience,
                })
                .send()
                .await?;
            let status = response.status().as_u16();
            let body = response.text().await?;
            if !(200..300).contains(&status) {
                let excerpt: String = body
                    .trim()
                    .replace(&base_token, "[redacted]")
                    .chars()
                    .take(200)
                    .collect();
                return Err(Error::Api {
                    status,
                    message: format!("the Vercel OIDC token exchange failed: {excerpt}"),
                });
            }
            serde_json::from_str::<VercelExchangeResponse>(&body)
                .ok()
                .and_then(|body| body.token)
                .map(|token| token.trim().to_string())
                .filter(|token| !token.is_empty())
                .ok_or_else(|| Error::Api {
                    status,
                    message: "the Vercel OIDC token exchange response has no token".to_string(),
                })
        }
    })
}

fn vercel_base_token() -> Result<String> {
    match env_value(VERCEL_TOKEN_FILE_ENV) {
        Some(path) => read_token_file(path),
        None => required_env(VERCEL_TOKEN_ENV),
    }
}

#[derive(serde::Serialize)]
struct VercelExchangeRequest<'a> {
    token: &'a str,
    aud: &'a str,
}

#[derive(serde::Deserialize)]
struct VercelExchangeResponse {
    token: Option<String>,
}

/// Reads a projected service account token from `path`
/// (default `/var/run/secrets/tokens/dodo-token`) for each exchange.
pub fn kubernetes(path: Option<&str>) -> SubjectTokenSource {
    file_source(
        "kubernetes",
        path.unwrap_or(KUBERNETES_TOKEN_PATH).to_owned(),
    )
}

/// Reads the token from `path` for each exchange. Without a path, reads the path from the
/// `DODO_PAYMENTS_IDENTITY_TOKEN_FILE` environment variable.
pub fn file(path: Option<&str>) -> SubjectTokenSource {
    match path {
        Some(path) => file_source("file", path.to_owned()),
        None => SubjectTokenSource::with_context("file", |_| {
            let value = required_env(crate::IDENTITY_TOKEN_FILE_ENV).and_then(read_token_file);
            async move { value }
        }),
    }
}

/// Calls `f` for each exchange. Use it with AWS STS `GetWebIdentityToken` or a similar call.
pub fn aws<F, Fut>(f: F) -> SubjectTokenSource
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<String>> + Send + 'static,
{
    let mut source = SubjectTokenSource::from_fn(f);
    source.kind = "aws";
    source
}

/// Always gives the same token. Use it for tests.
pub fn static_token(token: impl Into<String>) -> SubjectTokenSource {
    let token = token.into();
    SubjectTokenSource::with_context("static", move |_| {
        let token = token.clone();
        async move { Ok(token) }
    })
}

/// Finds a subject token source from the environment, in this order: GitHub Actions,
/// Terraform Cloud, the `DODO_PAYMENTS_IDENTITY_TOKEN_FILE` file, the Kubernetes default path.
pub fn auto() -> Option<SubjectTokenSource> {
    if env_value(GITHUB_REQUEST_URL_ENV).is_some() && env_value(GITHUB_REQUEST_TOKEN_ENV).is_some()
    {
        return Some(github_actions(None));
    }
    if env_value(TFC_TOKEN_ENV).is_some() || env_value(TFC_DODO_TOKEN_ENV).is_some() {
        return Some(terraform_cloud());
    }
    if let Some(path) = env_value(crate::IDENTITY_TOKEN_FILE_ENV) {
        return Some(file(Some(&path)));
    }
    if std::path::Path::new(KUBERNETES_TOKEN_PATH).is_file() {
        return Some(kubernetes(None));
    }
    None
}

fn file_source(kind: &'static str, path: String) -> SubjectTokenSource {
    SubjectTokenSource::with_context(kind, move |_| {
        let value = read_token_file(path.clone());
        async move { value }
    })
}

fn read_token_file(path: String) -> Result<String> {
    let contents = std::fs::read_to_string(&path).map_err(|error| {
        Error::Config(format!(
            "cannot read the identity token file {path}: {error}"
        ))
    })?;
    non_empty(contents, "the identity token file")
}

fn env_value(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn required_env(name: &str) -> Result<String> {
    env_value(name).ok_or_else(|| Error::Config(format!("missing {name} environment variable")))
}

fn non_empty(token: String, what: &str) -> Result<String> {
    let token = token.trim();
    if token.is_empty() {
        return Err(Error::Config(format!("{what} is empty")));
    }
    Ok(token.to_string())
}

async fn success(response: reqwest::Response) -> Result<reqwest::Response> {
    let status = response.status();
    if status.is_success() {
        return Ok(response);
    }
    Err(Error::Api {
        status: status.as_u16(),
        message: response.text().await.unwrap_or_default(),
    })
}

#[derive(serde::Deserialize)]
struct GithubIdToken {
    value: String,
}

#[derive(serde::Serialize)]
struct ExchangeRequest<'a> {
    grant_type: &'a str,
    subject_token_type: &'a str,
    subject_token: &'a str,
    identity_provider_id: &'a str,
}

#[derive(serde::Deserialize)]
struct ExchangeResponse {
    access_token: String,
    expires_in: Option<u64>,
    expires_at: Option<String>,
}

#[derive(serde::Deserialize)]
struct OAuthError {
    error: String,
    error_description: Option<String>,
}

struct CachedToken {
    access_token: String,
    refresh_at: Instant,
}

impl CachedToken {
    fn is_fresh(&self) -> bool {
        Instant::now() < self.refresh_at
    }
}

/// The access token cache of one client and its clones.
pub(crate) struct TokenCache {
    identity_provider_id: String,
    provider: SubjectTokenSource,
    // A tokio mutex holds concurrent requests while one exchange runs.
    token: tokio::sync::Mutex<Option<CachedToken>>,
}

impl fmt::Debug for TokenCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenCache")
            .field("identity_provider_id", &self.identity_provider_id)
            .field("provider", &self.provider)
            .finish_non_exhaustive()
    }
}

impl TokenCache {
    pub(crate) fn new(workload_identity: &WorkloadIdentity) -> Result<Self> {
        let identity_provider_id = workload_identity
            .identity_provider_id
            .clone()
            .filter(|value| !value.is_empty())
            .or_else(|| env_value(crate::IDENTITY_PROVIDER_ID_ENV))
            .ok_or_else(|| {
                Error::Config(format!(
                    "missing identity provider ID: set it on WorkloadIdentity or in the {} environment variable",
                    crate::IDENTITY_PROVIDER_ID_ENV
                ))
            })?;
        Ok(Self {
            identity_provider_id,
            provider: workload_identity.provider.clone(),
            token: tokio::sync::Mutex::new(None),
        })
    }

    pub(crate) async fn access_token(&self, client: &Client) -> Result<String> {
        let mut slot = self.token.lock().await;
        if let Some(cached) = slot.as_ref().filter(|cached| cached.is_fresh()) {
            return Ok(cached.access_token.clone());
        }
        let fresh = self.exchange(client).await?;
        let access_token = fresh.access_token.clone();
        *slot = Some(fresh);
        Ok(access_token)
    }

    /// Gets a new token after the API refused `rejected`, unless a concurrent request already did.
    pub(crate) async fn refresh_after_unauthorized(
        &self,
        client: &Client,
        rejected: &str,
    ) -> Result<String> {
        let mut slot = self.token.lock().await;
        if let Some(cached) = slot
            .as_ref()
            .filter(|cached| cached.access_token != rejected && cached.is_fresh())
        {
            return Ok(cached.access_token.clone());
        }
        *slot = None;
        let fresh = self.exchange(client).await?;
        let access_token = fresh.access_token.clone();
        *slot = Some(fresh);
        Ok(access_token)
    }

    pub(crate) async fn revoke(&self, client: &Client) -> Result<()> {
        let mut slot = self.token.lock().await;
        let Some(cached) = slot.take() else {
            return Ok(());
        };
        let request = client
            .http_client()
            .post(client.url(crate::WORKLOAD_IDENTITY_REVOKE_PATH))
            .bearer_auth(&cached.access_token);
        let response = client.send_token_request(request).await?;
        if response.status().is_success() {
            return Ok(());
        }
        Err(token_endpoint_error(response).await)
    }

    async fn exchange(&self, client: &Client) -> Result<CachedToken> {
        let context = SubjectTokenContext {
            base_url: client.base_url().to_string(),
            http: client.http_client().clone(),
        };
        let subject_token = self.provider.subject_token(context).await?;
        let request = client
            .http_client()
            .post(client.url(crate::WORKLOAD_IDENTITY_TOKEN_PATH))
            .json(&ExchangeRequest {
                grant_type: GRANT_TYPE,
                subject_token_type: SUBJECT_TOKEN_TYPE,
                subject_token: &subject_token,
                identity_provider_id: &self.identity_provider_id,
            });
        let response = client.send_token_request(request).await?;
        if !response.status().is_success() {
            return Err(token_endpoint_error(response).await);
        }
        let body: ExchangeResponse = response.json().await?;
        let lifetime = body
            .expires_at
            .as_deref()
            .and_then(parse_rfc3339)
            .map(|expires_at| {
                expires_at
                    .duration_since(SystemTime::now())
                    .unwrap_or(Duration::ZERO)
            })
            .or(body.expires_in.map(Duration::from_secs))
            .unwrap_or(Duration::ZERO);
        Ok(CachedToken {
            access_token: body.access_token,
            refresh_at: Instant::now()
                .checked_add(lifetime.saturating_sub(REFRESH_MARGIN))
                .unwrap_or_else(Instant::now),
        })
    }
}

/// Converts an RFC 6749 error response to [`Error::Api`] with `error: error_description`.
async fn token_endpoint_error(response: reqwest::Response) -> Error {
    let status = response.status().as_u16();
    let body = response.text().await.unwrap_or_default();
    let message = match serde_json::from_str::<OAuthError>(&body) {
        Ok(OAuthError {
            error,
            error_description: Some(description),
        }) => format!("{error}: {description}"),
        Ok(OAuthError { error, .. }) => error,
        Err(_) => body,
    };
    Error::Api { status, message }
}

/// Parses an RFC 3339 timestamp such as `2026-10-28T12:00:00.5+05:30`.
fn parse_rfc3339(value: &str) -> Option<SystemTime> {
    let number = |start: usize, end: usize| -> Option<i64> {
        let digits = value.get(start..end)?;
        if !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        digits.parse().ok()
    };
    let separator = |index: usize, allowed: &[u8]| {
        value
            .as_bytes()
            .get(index)
            .is_some_and(|byte| allowed.contains(byte))
    };
    if !(separator(4, b"-")
        && separator(7, b"-")
        && separator(10, b"Tt ")
        && separator(13, b":")
        && separator(16, b":"))
    {
        return None;
    }
    let (year, month, day) = (number(0, 4)?, number(5, 7)?, number(8, 10)?);
    let (hour, minute, second) = (number(11, 13)?, number(14, 16)?, number(17, 19)?);
    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || hour > 23
        || minute > 59
        || second > 60
    {
        return None;
    }

    let mut rest = value.get(19..)?;
    let mut nanos = 0u32;
    if let Some(fraction) = rest.strip_prefix('.') {
        let digits = fraction.bytes().take_while(u8::is_ascii_digit).count();
        if digits == 0 {
            return None;
        }
        nanos = format!("{:0<9}", &fraction[..digits.min(9)]).parse().ok()?;
        rest = &fraction[digits..];
    }
    let offset = match rest {
        "Z" | "z" => 0,
        _ => {
            let sign = match rest.as_bytes().first()? {
                b'+' => 1,
                b'-' => -1,
                _ => return None,
            };
            if rest.len() != 6 || rest.as_bytes()[3] != b':' {
                return None;
            }
            let hours: i64 = rest.get(1..3)?.parse().ok()?;
            let minutes: i64 = rest.get(4..6)?.parse().ok()?;
            sign * (hours * 3600 + minutes * 60)
        }
    };

    let seconds =
        days_from_civil(year, month, day) * 86_400 + hour * 3600 + minute * 60 + second - offset;
    Some(UNIX_EPOCH + Duration::new(u64::try_from(seconds).ok()?, nanos))
}

/// Days since 1970-01-01 in the proleptic Gregorian calendar.
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let day_of_year = (153 * ((month + 9) % 12) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unix(value: &str) -> Option<u64> {
        parse_rfc3339(value).map(|time| time.duration_since(UNIX_EPOCH).unwrap().as_secs())
    }

    #[test]
    fn parses_rfc3339_timestamps() {
        assert_eq!(unix("1970-01-01T00:00:00Z"), Some(0));
        assert_eq!(unix("2026-10-28T12:00:00Z"), Some(1_793_188_800));
        assert_eq!(unix("2026-10-28T12:00:00.123456Z"), Some(1_793_188_800));
        assert_eq!(unix("2026-10-28T17:30:00+05:30"), Some(1_793_188_800));
        assert_eq!(unix("2024-02-29T00:00:00-01:00"), Some(1_709_168_400));
    }

    #[test]
    fn rejects_invalid_timestamps() {
        assert_eq!(unix("2026-10-28"), None);
        assert_eq!(unix("2026-13-28T12:00:00Z"), None);
        assert_eq!(unix("2026-10-28T12:00:00"), None);
        assert_eq!(unix("2026-10-28T12:00:00+0530"), None);
    }

    #[test]
    fn default_audience_is_the_base_url_origin() {
        let context = SubjectTokenContext {
            base_url: "https://test.dodopayments.com/".to_string(),
            http: reqwest::Client::new(),
        };
        assert_eq!(context.default_audience(), "https://test.dodopayments.com");
    }
}
