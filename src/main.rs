use clap::Parser;
use futures::stream::{self, StreamExt};
use sitecheck::checker::{check_url, parse_urls_from_content, CheckResult, Status};
use sitecheck::cli::Cli;
use sitecheck::output;
use std::process::ExitCode;
use std::time::Duration;

fn collect_urls(cli: &Cli) -> Result<Vec<String>, std::io::Error> {
    let mut urls = cli.urls.clone();
    if let Some(path) = &cli.file {
        let content = std::fs::read_to_string(path)?;
        urls.extend(parse_urls_from_content(&content));
    }
    Ok(urls)
}

async fn run_once(cli: &Cli, urls: &[String]) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(cli.timeout))
        .user_agent("sitecheck/0.1")
        .build()?;

    let results: Vec<CheckResult> = stream::iter(urls.iter())
        .map(|u| check_url(&client, u, cli.retries))
        .buffer_unordered(cli.concurrency)
        .collect()
        .await;

    if cli.json {
        output::print_json(&results)?;
    } else {
        output::print_table(&results);
    }

    let all_ok = results.iter().all(|r| matches!(r.status, Status::Ok));
    Ok(all_ok)
}

fn unix_now() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    let urls = match collect_urls(&cli) {
        Ok(u) if !u.is_empty() => u,
        Ok(_) => {
            eprintln!("error: no URLs provided. Use positional args or --file.");
            return ExitCode::from(2);
        }
        Err(e) => {
            eprintln!("error: failed to read file: {e}");
            return ExitCode::from(2);
        }
    };

    if let Some(interval) = cli.watch {
        if interval == 0 {
            eprintln!("error: --watch must be greater than 0");
            return ExitCode::from(2);
        }
        loop {
            // In JSON mode, header goes to stderr to keep stdout clean
            eprintln!("\n=== check at unix:{} ===\n", unix_now());
            if let Err(e) = run_once(&cli, &urls).await {
                eprintln!("error: {e}");
            }
            tokio::time::sleep(Duration::from_secs(interval)).await;
        }
    }

    match run_once(&cli, &urls).await {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}
