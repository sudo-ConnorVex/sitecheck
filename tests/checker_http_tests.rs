use sitecheck::checker::{check_url, Status};
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn make_client(timeout_secs: u64) -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .unwrap()
}

#[tokio::test]
async fn check_url_returns_ok_on_200() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/ok"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = make_client(5).await;
    let url = format!("{}/ok", server.uri());
    let result = check_url(&client, &url, 0).await;

    assert_eq!(result.status, Status::Ok);
    assert_eq!(result.http_code, Some(200));
    assert!(result.response_ms.is_some());
    assert!(result.error.is_none());
}

#[tokio::test]
async fn check_url_returns_fail_on_500() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/error"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let client = make_client(5).await;
    let url = format!("{}/error", server.uri());
    let result = check_url(&client, &url, 0).await;

    assert_eq!(result.status, Status::Fail);
    assert_eq!(result.http_code, Some(500));
    assert!(result.error.is_some());
}

#[tokio::test]
async fn check_url_times_out() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/slow"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(5)))
        .mount(&server)
        .await;

    let client = make_client(1).await; // timeout 1 second
    let url = format!("{}/slow", server.uri());
    let result = check_url(&client, &url, 0).await;

    assert_eq!(result.status, Status::Fail);
    assert_eq!(result.error.as_deref(), Some("timeout"));
}

#[tokio::test]
async fn check_url_retries_then_succeeds() {
    let server = MockServer::start().await;

    // First call fails, second succeeds
    // wiremock lets us set expectations in order
    Mock::given(method("GET"))
        .and(path("/flaky"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/flaky"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = make_client(5).await;
    let url = format!("{}/flaky", server.uri());
    let result = check_url(&client, &url, 1).await; // 1 retry

    assert_eq!(result.status, Status::Ok);
    assert_eq!(result.http_code, Some(200));
}

#[tokio::test]
async fn check_url_invalid_url() {
    let client = make_client(5).await;
    let result = check_url(&client, "not-a-url", 0).await;

    assert_eq!(result.status, Status::Fail);
    assert_eq!(result.error.as_deref(), Some("invalid url"));
    assert!(result.http_code.is_none());
}
