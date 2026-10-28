use std::time::Duration;

use dodopayments::workload_identity::{self, WorkloadIdentity};
use dodopayments::{
    Client, ClientConfig, Error, WORKLOAD_IDENTITY_REVOKE_PATH, WORKLOAD_IDENTITY_TOKEN_PATH,
};
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

const SUBJECT_TOKEN: &str = "subject-token-for-tests";
const IDENTITY_PROVIDER_ID: &str = "wip_test";

fn exchange_body() -> serde_json::Value {
    json!({
        "grant_type": "urn:ietf:params:oauth:grant-type:token-exchange",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "subject_token": SUBJECT_TOKEN,
        "identity_provider_id": IDENTITY_PROVIDER_ID,
    })
}

fn token_response(access_token: &str, expires_in: u64) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({
        "access_token": access_token,
        "issued_token_type": "urn:ietf:params:oauth:token-type:access_token",
        "token_type": "Bearer",
        "expires_in": expires_in,
        "scope": "payments:read",
    }))
}

fn workload_identity() -> WorkloadIdentity {
    WorkloadIdentity::new(workload_identity::static_token(SUBJECT_TOKEN))
        .with_identity_provider_id(IDENTITY_PROVIDER_ID)
}

fn client(server: &MockServer) -> Client {
    Client::new(ClientConfig::new(server.uri()).with_workload_identity(workload_identity()))
        .expect("client builds")
}

fn exchange() -> wiremock::MockBuilder {
    Mock::given(method("POST")).and(path(WORKLOAD_IDENTITY_TOKEN_PATH))
}

fn api_call(access_token: &str) -> wiremock::MockBuilder {
    Mock::given(method("GET"))
        .and(path("/payments"))
        .and(header("authorization", format!("Bearer {access_token}")))
}

fn no_authorization(request: &Request) -> bool {
    !request.headers.contains_key("authorization")
}

async fn list_payments(client: &Client) -> reqwest::Response {
    client
        .send(client.request(reqwest::Method::GET, "/payments"))
        .await
        .expect("request is sent")
}

#[tokio::test]
async fn exchange_then_api_call_sends_access_token() {
    let server = MockServer::start().await;
    exchange()
        .and(header("content-type", "application/json"))
        .and(body_json(exchange_body()))
        .and(no_authorization)
        .respond_with(token_response("access-1", 3600))
        .expect(1)
        .mount(&server)
        .await;
    api_call("access-1")
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "items": [] })))
        .expect(1)
        .mount(&server)
        .await;

    let response = list_payments(&client(&server)).await;

    assert_eq!(response.status(), 200);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_calls_share_one_exchange() {
    let server = MockServer::start().await;
    exchange()
        .respond_with(token_response("access-1", 3600).set_delay(Duration::from_millis(300)))
        .expect(1)
        .mount(&server)
        .await;
    api_call("access-1")
        .respond_with(ResponseTemplate::new(200))
        .expect(2)
        .mount(&server)
        .await;
    let client = client(&server);

    let first = tokio::spawn({
        let client = client.clone();
        async move { list_payments(&client).await.status() }
    });
    let second = tokio::spawn({
        let client = client.clone();
        async move { list_payments(&client).await.status() }
    });

    assert_eq!(first.await.expect("task completes"), 200);
    assert_eq!(second.await.expect("task completes"), 200);
}

#[tokio::test]
async fn token_with_five_minutes_or_less_left_is_exchanged_again() {
    let server = MockServer::start().await;
    exchange()
        .respond_with(token_response("access-1", 200))
        .expect(2)
        .mount(&server)
        .await;
    api_call("access-1")
        .respond_with(ResponseTemplate::new(200))
        .expect(2)
        .mount(&server)
        .await;
    let client = client(&server);

    list_payments(&client).await;
    list_payments(&client).await;
}

#[tokio::test]
async fn token_with_more_than_five_minutes_left_is_reused() {
    let server = MockServer::start().await;
    exchange()
        .respond_with(token_response("access-1", 3600))
        .expect(1)
        .mount(&server)
        .await;
    api_call("access-1")
        .respond_with(ResponseTemplate::new(200))
        .expect(2)
        .mount(&server)
        .await;
    let client = client(&server);

    list_payments(&client).await;
    list_payments(&client).await;
}

#[tokio::test]
async fn api_401_exchanges_and_retries_once_then_surfaces() {
    let server = MockServer::start().await;
    exchange()
        .respond_with(token_response("access-1", 3600))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;
    exchange()
        .respond_with(token_response("access-2", 3600))
        .expect(1)
        .mount(&server)
        .await;
    api_call("access-1")
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&server)
        .await;
    api_call("access-2")
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&server)
        .await;

    let response = list_payments(&client(&server)).await;

    assert_eq!(response.status(), 401);
}

#[test]
fn api_key_with_workload_identity_is_a_config_error() {
    let result = Client::new(
        ClientConfig::new("https://test.dodopayments.com")
            .with_api_key("My API Key")
            .with_workload_identity(workload_identity()),
    );

    assert!(matches!(result, Err(Error::Config(_))));
}

#[tokio::test]
async fn exchange_invalid_grant_is_a_permission_error() {
    let server = MockServer::start().await;
    exchange()
        .respond_with(ResponseTemplate::new(403).set_body_json(json!({
            "error": "invalid_grant",
            "error_description": "no mapping matches the token claims",
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&server)
        .await;
    let client = client(&server);

    let error = client
        .send(client.request(reqwest::Method::GET, "/payments"))
        .await
        .expect_err("the exchange is refused");

    match error {
        Error::Api { status, message } => {
            assert_eq!(status, 403);
            assert_eq!(
                message,
                "invalid_grant: no mapping matches the token claims"
            );
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[tokio::test]
async fn revoke_sends_access_token_and_clears_cache() {
    let server = MockServer::start().await;
    exchange()
        .respond_with(token_response("access-1", 3600))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;
    exchange()
        .respond_with(token_response("access-2", 3600))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(WORKLOAD_IDENTITY_REVOKE_PATH))
        .and(header("authorization", "Bearer access-1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    api_call("access-1")
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;
    api_call("access-2")
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;
    let client = client(&server);
    list_payments(&client).await;

    client
        .revoke_workload_identity()
        .await
        .expect("revoke succeeds");
    client
        .revoke_workload_identity()
        .await
        .expect("revoke without a cached token does nothing");
    let response = list_payments(&client).await;

    assert_eq!(response.status(), 200);
}
