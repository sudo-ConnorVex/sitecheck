use clap::Parser;
use sitecheck::cli::Cli;

#[test]
fn concurrency_default_is_20() {
    let cli = Cli::try_parse_from(["sitecheck", "https://example.com"]).unwrap();
    assert_eq!(cli.concurrency, 20);
}

#[test]
fn concurrency_zero_is_rejected() {
    let result = Cli::try_parse_from(["sitecheck", "https://example.com", "--concurrency", "0"]);
    assert!(result.is_err());
}

#[test]
fn concurrency_too_large_is_rejected() {
    let result = Cli::try_parse_from(["sitecheck", "https://example.com", "--concurrency", "201"]);
    assert!(result.is_err());
}

#[test]
fn concurrency_valid_range() {
    let cli =
        Cli::try_parse_from(["sitecheck", "https://example.com", "--concurrency", "50"]).unwrap();
    assert_eq!(cli.concurrency, 50);
}
