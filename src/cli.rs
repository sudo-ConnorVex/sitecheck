use clap::Parser;
use std::path::PathBuf;

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

    /// Maximum number of concurrent requests
    #[arg(long, default_value_t = 20)]
    pub concurrency: usize,

    /// Machine-readable JSON output
    #[arg(long, default_value_t = false)]
    pub json: bool,

    /// Repeat the check every N seconds (watch mode)
    #[arg(long, value_name = "SECONDS")]
    pub watch: Option<u64>,
}
