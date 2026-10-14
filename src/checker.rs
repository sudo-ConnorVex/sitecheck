use serde::Serialize;
// use serde_json::to_string;
use std::{
    error::Error,
    time::{Duration, Instant},
};
use url::Url;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Fail,
}

#[derive(Debug, Serialize, Clone)]
pub struct CheckResult {
    pub url: String,
    pub status: Status,
    pub http_code: Option<u16>,
    pub response_ms: Option<u128>,
    pub error: Option<String>,
}

/// Basic URL validation. Only http:// and https:// are accepted.
pub fn is_valid_url(url: &str) -> bool {
    match Url::parse(url.trim()) {
        Ok(parsed) => matches!(parsed.scheme(), "http" | "https"),
        Err(_) => false,
    }
}

/// Categorize a reqwest error into a short readable label.
fn classify_error(e: &reqwest::Error) -> String {
    if e.is_timeout() {
        return "timeout".to_string();
    }
    let msg = e.to_string().to_lowercase();
    if e.is_connect() {
        let mut source: Option<&(dyn Error + 'static)> = e.source();
        while let Some(err) = source {
            let msg = err.to_string().to_lowercase();
            if msg.contains("dns")
                || msg.contains("resolve")
                || msg.contains("name or service")
                || msg.contains("nodename")
                || msg.contains("no such host")
            {
                return "dns".to_string();
            }
            source = err.source();
        }
        return "connection".to_string();
    }
    if e.is_request() {
        return "invalid request".to_string();
    }
    if msg.contains("certificate") || msg.contains("tls") || msg.contains("ssl") {
        return "tls".to_string();
    }
    format!("other: {}", e)
}

pub async fn check_url(client: &reqwest::Client, url: &str, retries: u32) -> CheckResult {
    if !is_valid_url(url) {
        return CheckResult {
            url: url.to_string(),
            status: Status::Fail,
            http_code: None,
            response_ms: None,
            error: Some("invalid url".to_string()),
        };
    }

    let attempts = retries.saturating_add(1);
    let mut last_err: Option<String> = None;
    let mut last_code: Option<u16> = None;
    let mut last_ms: Option<u128> = None;

    for attempt in 0..attempts {
        let start = Instant::now();
        match client.get(url).send().await {
            Ok(resp) => {
                let elapsed = start.elapsed().as_millis();
                let code = resp.status().as_u16();
                let is_ok = resp.status().is_success() || resp.status().is_redirection();

                if is_ok {
                    return CheckResult {
                        url: url.to_string(),
                        status: Status::Ok,
                        http_code: Some(code),
                        response_ms: Some(elapsed),
                        error: None,
                    };
                }

                // HTTP error response (4xx or 5xx)
                last_err = Some(format!("HTTP {}", code));
                last_code = Some(code);
                last_ms = Some(elapsed);

                // Retry only on 5xx server errors; 4xx is a client issue
                if !resp.status().is_server_error() {
                    break;
                }

                if attempt + 1 < attempts {
                    tokio::time::sleep(Duration::from_millis(300)).await;
                }
            }
            Err(e) => {
                last_err = Some(classify_error(&e));
                if attempt + 1 < attempts {
                    tokio::time::sleep(Duration::from_millis(300)).await;
                }
            }
        }
    }

    CheckResult {
        url: url.to_string(),
        status: Status::Fail,
        http_code: last_code,
        response_ms: last_ms,
        error: last_err,
    }
}

/// Parse URLs from raw file content. Ignores empty lines and lines starting with '#'.
pub fn parse_urls_from_content(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.to_string())
        .collect()
}
