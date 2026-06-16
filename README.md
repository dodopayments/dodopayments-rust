# Dodo Payments Rust API library

The Dodo Payments Rust library provides convenient access to the Dodo Payments REST API from applications written in Rust.

The REST API documentation can be found on [docs.dodopayments.com](https://docs.dodopayments.com/api-reference/introduction). The full API of this library can be found in [api.md](api.md).

## Installation

This crate is published from [dodopayments/dodopayments-rust](https://github.com/dodopayments/dodopayments-rust). Add it to your `Cargo.toml`:

```toml
[dependencies]
dodopayments = { git = "https://github.com/dodopayments/dodopayments-rust" }
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
    let result = client.checkout_sessions_create(&Default::default()).await?;
    println!("{result:?}");
    Ok(())
}
```

You can also configure the client explicitly:

```rust
use dodopayments::{Client, ClientConfig};

let client = Client::new(ClientConfig::new("https://test.dodopayments.com").with_api_key("My API Key"))?;
```

### Pagination

List endpoints return a typed page whose `items` field holds the current page of results. Fetch the next page with `get_next_page`, or stream every item across all pages with `into_stream`:

```rust
use futures::StreamExt;

let mut items = Box::pin(client.payments_list(None).await?.into_stream());
while let Some(item) = items.next().await {
    let item = item?;
    println!("{item:?}");
}
```

Or advance one page at a time:

```rust
let mut page = client.payments_list(None).await?;
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
match client.checkout_sessions_create(&Default::default()).await {
    Ok(result) => println!("{result:?}"),
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
    ClientConfig::new("https://test.dodopayments.com")
        .with_api_key("My API Key")
        .with_timeout(Duration::from_secs(60)),
)?;
```

### Environments

| Name | Base URL |
| --- | --- |
| `test_mode` | https://test.dodopayments.com |
| `live_mode` | https://live.dodopayments.com |

Pass the desired base URL to `ClientConfig::new`.

### Undocumented endpoints

To call an endpoint not yet exposed as a typed method, use the low-level `request` builder, which applies authentication and the base URL:

```rust
let response = client
    .request(reqwest::Method::GET, "/some/path")
    .send()
    .await?;
```

## Semantic versioning

This package follows [SemVer](https://semver.org/spec/v2.0.0.html) conventions. As the library is in initial development (`0.x`), minor version bumps may include breaking changes until `1.0.0`.

We take backwards compatibility seriously and work hard to ensure that breaking changes are clearly communicated in the [changelog](CHANGELOG.md).

## Contributing

See [the contributing documentation](./CONTRIBUTING.md).
