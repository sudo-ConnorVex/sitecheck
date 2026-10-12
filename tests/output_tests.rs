use sitecheck::checker::{CheckResult, Status};

#[test]
fn json_serialization_ok() {
    let r = CheckResult {
        url: "https://x.com".into(),
        status: Status::Ok,
        http_code: Some(200),
        response_ms: Some(120),
        error: None,
    };
    let s = serde_json::to_string(&r).unwrap();
    assert!(s.contains("\"status\":\"ok\""));
    assert!(s.contains("\"http_code\":200"));
}

#[test]
fn json_serialization_fail() {
    let r = CheckResult {
        url: "https://bad.invalid".into(),
        status: Status::Fail,
        http_code: None,
        response_ms: None,
        error: Some("dns".into()),
    };
    let s = serde_json::to_string(&r).unwrap();
    assert!(s.contains("\"status\":\"fail\""));
    assert!(s.contains("\"error\":\"dns\""));
    assert!(s.contains("\"http_code\":null"));
}
