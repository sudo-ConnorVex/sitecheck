# sitecheck

A small Rust CLI that checks the health of multiple websites concurrently.

## Build

```bash
cargo build --release
```

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

# Watch mode (repeat every 30 seconds)
./target/release/sitecheck --file urls.txt --watch 30
```

## Sample output

```
+---------------------+--------+------+-----------+-------+
| URL                 | STATUS | HTTP | TIME (ms) | ERROR |
+---------------------+--------+------+-----------+-------+
| https://example.com | ok     | 200  | 143       |       |
| https://bad.invalid | fail   | -    | -         | dns   |
+---------------------+--------+------+-----------+-------+
```

## Exit codes

| Code | Meaning                        |
|------|--------------------------------|
| 0    | All URLs are healthy           |
| 1    | At least one URL failed        |
| 2    | Invalid input or runtime error |

## Project layout

- `src/cli.rs` — argument parsing with clap
- `src/checker.rs` — per-URL checking, validation, error classification
- `src/output.rs` — table and JSON formatting
- `src/main.rs` — orchestration, exit codes, watch mode
- `tests/` — integration tests

## Options

| Flag            | Default | Description                              |
|-----------------|---------|------------------------------------------|
| `--file <FILE>` | -       | Read URLs from a text file               |
| `--timeout <S>` | 10      | Per-request timeout in seconds           |
| `--retries <N>` | 0       | Retry attempts on failure                |
| `--json`        | false   | Emit JSON instead of a table             |
| `--watch <S>`   | -       | Repeat the check every N seconds         |
