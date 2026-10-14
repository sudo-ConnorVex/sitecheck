# sitecheck

A small Rust CLI that checks the health of multiple websites concurrently.

## Build

```bash
cargo build --release
```

The binary is placed at `target/release/sitecheck`.

## Usage

```bash
# From positional arguments
./target/release/sitecheck https://example.com https://www.rust-lang.org

# From a file (one URL per line, '#' starts a comment)
./target/release/sitecheck --file urls.txt

# JSON output
./target/release/sitecheck --file urls.txt --json

# Timeout and retries
./target/release/sitecheck https://example.com --timeout 5 --retries 2

# Limit concurrency
./target/release/sitecheck --file urls.txt --concurrency 50

# Watch mode (repeat every 30 seconds)
./target/release/sitecheck --file urls.txt --watch 30
```

## Options

| Flag                    | Default | Description                                        |
|-------------------------|---------|----------------------------------------------------|
| `--file <FILE>`         | -       | Read URLs from a text file (one per line)          |
| `--timeout <SECONDS>`   | 10      | Per-request timeout                                |
| `--retries <N>`         | 0       | Retry attempts on transient failures               |
| `--concurrency <N>`     | 20      | Maximum concurrent requests (1-200)                |
| `--json`                | false   | Emit JSON instead of a table                       |
| `--watch <SECONDS>`     | -       | Repeat the check every N seconds                   |

## Sample output

```
┌───────────────────────────┬────────┬──────┬───────────┬───────┐
│ URL                       ┆ STATUS ┆ HTTP ┆ TIME (ms) ┆ ERROR │
╞═══════════════════════════╪════════╪══════╪═══════════╪═══════╡
│ https://example.com       ┆ ok     ┆ 200  ┆ 442       ┆       │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ https://www.rust-lang.org ┆ ok     ┆ 200  ┆ 810       ┆       │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ https://bad.invalid       ┆ fail   ┆ -    ┆ -         ┆ dns   │
└───────────────────────────┴────────┴──────┴───────────┴───────┘
```

## Exit codes

| Code | Meaning                                              |
|------|------------------------------------------------------|
| 0    | All URLs are healthy                                 |
| 1    | At least one URL failed (invalid, timeout, DNS, ...) |
| 2    | Application error (no input, file not found, ...)    |

## Retry behavior

Retries are triggered only on **transient failures**:

- Network errors (DNS, timeout, connection refused)
- HTTP 5xx (server errors)

HTTP 4xx responses (client errors like 404, 401) are **not** retried, because repeating the request won't change the outcome.

Default delay between retries is 300ms.

## Concurrency

Requests run concurrently with a configurable cap. The default cap is 20 concurrent requests, adjustable via `--concurrency` (range 1–200). Results are returned in the same order as the input URLs.

## Project structure

```
src/
├── main.rs      # Entry point, orchestration, watch mode
├── lib.rs       # Library root
├── cli.rs       # CLI argument definitions (clap)
├── checker.rs   # URL validation and HTTP checks
└── output.rs    # Table and JSON output
tests/
├── checker_tests.rs
├── checker_http_tests.rs
├── cli_tests.rs
└── output_tests.rs
```

## Tests

```bash
cargo test
```

Test suites:

- `checker_tests` — URL validation and file parsing
- `checker_http_tests` — HTTP behavior with a local mock server (wiremock)
- `cli_tests` — CLI argument validation
- `output_tests` — JSON serialization
