#![allow(dead_code)]

mod client;
mod config;
mod error;
pub mod models;
mod resources;

pub use client::Client;
pub use config::ClientConfig;
pub use error::{Error, Result};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_BASE_URL: &str = "https://test.dodopayments.com";
pub(crate) const API_KEY_ENV: &str = "DODO_PAYMENTS_API_KEY";
