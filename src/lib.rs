#![allow(dead_code)]

mod client;
mod config;
mod error;
pub mod models;
mod resources;
pub mod workload_identity;

pub use client::Client;
pub use config::ClientConfig;
pub use error::{Error, Result};
pub use workload_identity::WorkloadIdentity;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_BASE_URL: &str = "https://live.dodopayments.com";
pub const LIVE_MODE_BASE_URL: &str = "https://live.dodopayments.com";
pub const TEST_MODE_BASE_URL: &str = "https://test.dodopayments.com";
pub(crate) const API_KEY_ENV: &str = "DODO_PAYMENTS_API_KEY";
pub(crate) const BASE_URL_ENV: &str = "DODO_PAYMENTS_BASE_URL";
pub const WORKLOAD_IDENTITY_TOKEN_PATH: &str = "/workload-identity/token";
pub const WORKLOAD_IDENTITY_REVOKE_PATH: &str = "/workload-identity/token/revoke";
pub(crate) const IDENTITY_PROVIDER_ID_ENV: &str = "DODO_PAYMENTS_IDENTITY_PROVIDER_ID";
pub(crate) const IDENTITY_TOKEN_FILE_ENV: &str = "DODO_PAYMENTS_IDENTITY_TOKEN_FILE";
pub(crate) const ID_TOKEN_ENV: &str = "DODO_PAYMENTS_ID_TOKEN";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Environment {
    LiveMode,
    TestMode,
}

impl Environment {
    pub fn base_url(self) -> &'static str {
        match self {
            Environment::LiveMode => LIVE_MODE_BASE_URL,
            Environment::TestMode => TEST_MODE_BASE_URL,
        }
    }
}
