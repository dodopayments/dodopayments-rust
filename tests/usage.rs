use dodopayments::{Client, ClientConfig};

#[tokio::test]
async fn usage() {
    let base_url = match std::env::var("TEST_API_BASE_URL") {
        Ok(url) => url,
        Err(_) => return,
    };
    let client = Client::new(ClientConfig::new(base_url).with_api_key("My API Key")).unwrap();
    let _ = client
        .checkout_sessions_create(&serde_json::json!({}))
        .await;
}
