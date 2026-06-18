# Dodo Payments Rust API library

The Dodo Payments Rust library provides convenient access to the Dodo Payments REST API from applications written in Rust.

The REST API documentation can be found on [docs.dodopayments.com](https://docs.dodopayments.com/api-reference/introduction). The full API of this library can be found in [api.md](api.md).

## Installation

```sh
cargo add dodopayments
```

Or add it to your `Cargo.toml` manually:

```toml
[dependencies]
dodopayments = "1.105.2"
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

## Requirements

Rust 1.75 or later.

## Usage

The client reads your API key from the `DODO_PAYMENTS_API_KEY` environment variable by default:

```rust
use dodopayments::Client;

#[tokio::main]
async fn main() -> dodopayments::Result<()> {
    let client = Client::from_env()?;
    let result = client
        .checkout_sessions()
        .create()
        .body(dodopayments::models::CheckoutSessionsCreateParams {
                cancel_url: Some("cancel_url".to_string()),
                confirm: Some(true),
                customer_business_name: Some("customer_business_name".to_string()),
                discount_code: Some("discount_code".to_string()),
                discount_codes: Some(vec!["string".to_string()]),
                force_3ds: Some(true),
                mandate_min_amount_inr_paise: Some(0),
                metadata: Some(std::collections::HashMap::from([("foo".to_string(), "string".to_string())])),
                minimal_address: Some(true),
                payment_method_id: Some("payment_method_id".to_string()),
                product_collection_id: Some("product_collection_id".to_string()),
                return_url: Some("return_url".to_string()),
                short_link: Some(true),
                show_saved_payment_methods: Some(true),
                tax_id: Some("tax_id".to_string()),
                ..Default::default()
            })
        .await?;
    println!("{result:?}");
    Ok(())
}
```

You can also configure the client explicitly. `Client::new` returns a `Result`, so unwrap it with `?` inside a function that returns `dodopayments::Result`:

```rust
use dodopayments::{Client, ClientConfig};

#[tokio::main]
async fn main() -> dodopayments::Result<()> {
    let client = Client::new(ClientConfig::new("https://live.dodopayments.com").with_api_key("My API Key"))?;
    println!("{}", client.base_url());
    Ok(())
}
```

### Pagination

List endpoints return a typed page whose `items` field holds the current page of results. Fetch the next page with `get_next_page`, or stream every item across all pages with `into_stream`:

```rust
use futures::StreamExt;

let mut items = Box::pin(client
    .payments()
    .list()
    .query(serde_json::json!({}))
    .await?.into_stream());
while let Some(item) = items.next().await {
    let item = item?;
    println!("{item:?}");
}
```

Or advance one page at a time:

```rust
let mut page = client
    .payments()
    .list()
    .query(serde_json::json!({}))
    .await?;
loop {
    for item in &page.items {
        println!("{item:?}");
    }
    match page.get_next_page().await? {
        Some(next) => page = next,
        None => break,
    }
}
```

### Handling errors

Every method returns a `dodopayments::Result<T>`. Failures are represented by the `dodopayments::Error` enum:

```rust
let result = client
    .checkout_sessions()
    .create()
    .body(dodopayments::models::CheckoutSessionsCreateParams {
            cancel_url: Some("cancel_url".to_string()),
            confirm: Some(true),
            customer_business_name: Some("customer_business_name".to_string()),
            discount_code: Some("discount_code".to_string()),
            discount_codes: Some(vec!["string".to_string()]),
            force_3ds: Some(true),
            mandate_min_amount_inr_paise: Some(0),
            metadata: Some(std::collections::HashMap::from([("foo".to_string(), "string".to_string())])),
            minimal_address: Some(true),
            payment_method_id: Some("payment_method_id".to_string()),
            product_collection_id: Some("product_collection_id".to_string()),
            return_url: Some("return_url".to_string()),
            short_link: Some(true),
            show_saved_payment_methods: Some(true),
            tax_id: Some("tax_id".to_string()),
            ..Default::default()
        })
    .await;

match result {
    Ok(value) => println!("{value:?}"),
    Err(dodopayments::Error::Api { status, message }) => {
        eprintln!("API returned {status}: {message}");
    }
    Err(err) => eprintln!("request failed: {err}"),
}
```

### Timeouts

The default request timeout is 30 seconds. Override it per client:

```rust
use std::time::Duration;
use dodopayments::{Client, ClientConfig};

let client = Client::new(
    ClientConfig::new("https://live.dodopayments.com")
        .with_api_key("My API Key")
        .with_timeout(Duration::from_secs(60)),
)?;
```

### Environments

| Name        | Base URL                      |
| ----------- | ----------------------------- |
| `live_mode` | https://live.dodopayments.com |
| `test_mode` | https://test.dodopayments.com |

The default base URL is `https://live.dodopayments.com`. Select another environment with the `Environment` enum instead of hard-coding URLs:

```rust
use dodopayments::{Client, ClientConfig, Environment};

let client = Client::new(
    ClientConfig::from_environment(Environment::TestMode).with_api_key("My API Key"),
)?;
```

To keep reading the API key from `DODO_PAYMENTS_API_KEY` via `from_env()` while targeting a non-default environment, override it on the config:

```rust
use dodopayments::{Client, ClientConfig, Environment};

let client = Client::new(ClientConfig::from_env()?.with_environment(Environment::TestMode))?;
```

Alternatively, `Client::from_env()` uses the default base URL unless you set the `DODO_PAYMENTS_BASE_URL` environment variable, e.g. `DODO_PAYMENTS_BASE_URL=https://test.dodopayments.com` to target `test_mode`.

### Undocumented endpoints

To call an endpoint not yet exposed as a typed method, use the low-level `request` builder, which applies authentication and the base URL:

```rust
let response = client
    .request(reqwest::Method::GET, "/some/path")
    .send()
    .await?;
```

## Semantic versioning

This package follows [SemVer](https://semver.org/spec/v2.0.0.html) conventions. Breaking changes are released in major versions; minor and patch versions remain backwards compatible.

We take backwards compatibility seriously and work hard to ensure that breaking changes are clearly communicated in the [changelog](CHANGELOG.md).

## Contributing

See [the contributing documentation](./CONTRIBUTING.md).
