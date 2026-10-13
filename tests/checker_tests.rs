use sitecheck::checker::{is_valid_url, parse_urls_from_content};

#[test]
fn valid_http_and_https() {
    assert!(is_valid_url("http://example.com"));
    assert!(is_valid_url("https://example.com"));
    assert!(is_valid_url("https://sub.example.com/path?q=1"));
    assert!(is_valid_url("http://localhost:3000"));
    assert!(is_valid_url("http://192.168.1.10"));
    assert!(is_valid_url("http://127.0.0.1"));
}

#[test]
fn invalid_urls() {
    assert!(!is_valid_url(""));
    assert!(!is_valid_url("example.com"));
    assert!(!is_valid_url("ftp://example.com"));
    assert!(!is_valid_url("https://"));
}

#[test]
fn parse_file_ignores_comments_and_blanks() {
    let content = "\
https://a.com
# this is a comment

https://b.com
   https://c.com
";
    let urls = parse_urls_from_content(content);
    assert_eq!(
        urls,
        vec!["https://a.com", "https://b.com", "https://c.com"]
    );
}

#[test]
fn parse_file_empty() {
    assert!(parse_urls_from_content("").is_empty());
    assert!(parse_urls_from_content("# only comments\n\n").is_empty());
}
