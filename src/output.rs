use crate::checker::{CheckResult, Status};
use comfy_table::{presets::UTF8_FULL, Cell, Color, Table};

pub fn print_table(results: &[CheckResult]) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["URL", "STATUS", "HTTP", "TIME (ms)", "ERROR"]);

    for r in results {
        let (status_txt, color) = match r.status {
            Status::Ok => ("ok", Color::Green),
            Status::Fail => ("fail", Color::Red),
        };
        table.add_row(vec![
            Cell::new(&r.url),
            Cell::new(status_txt).fg(color),
            Cell::new(
                r.http_code
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "-".into()),
            ),
            Cell::new(
                r.response_ms
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "-".into()),
            ),
            Cell::new(r.error.clone().unwrap_or_default()),
        ]);
    }

    println!("{table}");
}

pub fn print_json(results: &[CheckResult]) -> Result<(), serde_json::Error> {
    println!("{}", serde_json::to_string_pretty(results)?);
    Ok(())
}
