use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use crate::metrics::Metrics;
use crate::parser::LogRow;

static DURATION_NEXT_RETRY: Duration = Duration::from_millis(50);

pub fn attach_file<P>(filename: P, metrics: Metrics) -> Result<(), io::Error>
where
    P: AsRef<Path> + Display,
{
    log::info!("Processing data from: {}", filename);
    let file = File::open(filename)?;

    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) if bytes_read == 0 => {
                // Be nice with CPU
                thread::sleep(DURATION_NEXT_RETRY);
            }
            Ok(_) => {
                // Process line
                if let Ok(row) = LogRow::from_str(&line) {
                    log::debug!("{:?}", row);

                    // Update metrics
                    metrics
                        .http_response_code_total
                        .with_label_values(&[
                            &row.request_method,
                            &row.request_path,
                            &row.request_protocol,
                            &format!("{}", row.response_status),
                        ])
                        .inc();

                    metrics
                        .http_response_body_size_total
                        .with_label_values(&[
                            &row.request_method,
                            &row.request_path,
                            &row.request_protocol,
                        ])
                        .inc_by(row.response_body_bytes_sent);
                } else {
                    log::warn!("Fail to process line: {}", line);
                }

                line.clear();
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
}
