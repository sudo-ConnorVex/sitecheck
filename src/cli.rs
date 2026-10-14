use clap::Parser;
use std::path::PathBuf;

// Checking the range of the concurrency option number
fn concurrency_in_range(s: &str) -> Result<usize, String> {
    let value: usize = s
        .parse()
        .map_err(|_| format!("`{s}` is not a valid number"))?;
    if (1..=200).contains(&value) {
        Ok(value)
    } else {
        Err(format!(
            "concurrency must be between 1 and 200, got {value}"
        ))
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "sitecheck",
    version,
    about = "Check the health of multiple websites"
)]
pub struct Cli {
    /// URLs passed as positional arguments
    #[arg(value_name = "URL")]
    pub urls: Vec<String>,

    /// Text file with one URL per line
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Per-request timeout in seconds
    #[arg(long, default_value_t = 10)]
    pub timeout: u64,

    /// Number of retries on failure
    #[arg(long, default_value_t = 0)]
    pub retries: u32,

    /// Maximum number of concurrent requests (1 - 200)
    #[arg(long, default_value_t = 20, value_parser = concurrency_in_range)]
    pub concurrency: usize,

    /// Machine-readable JSON output
    #[arg(long, default_value_t = false)]
    pub json: bool,

    /// Repeat the check every N seconds (watch mode)
    #[arg(long, value_name = "SECONDS")]
    pub watch: Option<u64>,
}
