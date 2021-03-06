use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use crate::metrics::Metrics;
use crate::parser::LogRow;

static DURATION_NEXT_RETRY: Duration = Duration::from_millis(50);

#[derive(Debug)]
pub struct EnabledMetric {
    pub response_status_short: bool,
    pub response_status_full: bool,
    pub response_size: bool,
}

fn process_line(line: &str, metrics: &Metrics, enabled: &EnabledMetric) {
    if let Ok(row) = LogRow::from_str(&line) {
        log::debug!("{:?}", row);

        let status = format!("{}", row.response_status);

        if enabled.response_status_short {
            metrics
                .http_response_total
                .with_label_values(&[&status])
                .inc();
        }

        if enabled.response_status_full {
            metrics
                .http_response_code_total
                .with_label_values(&[
                    &row.request_method,
                    &row.request_path,
                    &row.request_protocol,
                    &status,
                ])
                .inc();
        }

        if enabled.response_size {
            metrics
                .http_response_body_size_total
                .with_label_values(&[
                    &row.request_method,
                    &row.request_path,
                    &row.request_protocol,
                ])
                .inc_by(row.response_body_bytes_sent);
        }
    } else {
        log::warn!("Fail to process line: {}", line);

        metrics.parse_error.inc();
    }
}

pub fn attach_file<P>(
    filename: P,
    metrics: Metrics,
    enabled: EnabledMetric,
) -> Result<(), io::Error>
where
    P: AsRef<Path> + Display,
{
    log::info!("Processing data from: {}", filename);
    let mut file = File::open(filename)?;
    file.seek(SeekFrom::End(0))?;

    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) if bytes_read == 0 => {
                // Be nice with CPU
                thread::sleep(DURATION_NEXT_RETRY);
            }
            Ok(_) => {
                process_line(&line, &metrics, &enabled);

                line.clear();
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus::{Encoder, TextEncoder};

    macro_rules! metrics {
        () => {{
            let registry = prometheus::Registry::new();
            let metrics_collection = crate::metrics::Metrics::new("test");
            metrics_collection.register(&registry);
            (registry, metrics_collection)
        }};
    }

    macro_rules! gather_metrics {
        ($registry:expr) => {{
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            encoder.encode(&$registry.gather(), &mut buffer).unwrap();
            String::from_utf8(buffer).unwrap()
        }};
    }

    macro_rules! enabled_metrics {
        () => {
            EnabledMetric {
                response_status_short: true,
                response_status_full: true,
                response_size: true,
            }
        };
    }

    #[test]
    fn test_valid_line() {
        let (registry, metrics_collection) = metrics!();
        let line = r#"192.168.1.84 - - [22/Jan/2021:17:24:13 +0000] "GET /favicon.ico HTTP/1.1" 404 134 "http://serv-lemoigne/" "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:84.0) Gecko/20100101 Firefox/84.0""#;

        process_line(line, &metrics_collection, &enabled_metrics!());

        assert_eq!(
            gather_metrics!(registry),
            "# HELP test_http_response_body_size_total Size of HTTP request per request info\n\
             # TYPE test_http_response_body_size_total counter\n\
             test_http_response_body_size_total{method=\"GET\",path=\"/favicon.ico\",protocol=\"HTTP/1.1\"} 134\n\
             # HELP test_http_response_code_total Count of HTTP request per request info and response code\n\
             # TYPE test_http_response_code_total counter\n\
             test_http_response_code_total{method=\"GET\",path=\"/favicon.ico\",protocol=\"HTTP/1.1\",status=\"404\"} 1\n\
             # HELP test_http_response_total Number of HTTP request by status code\n\
             # TYPE test_http_response_total counter\n\
             test_http_response_total{status=\"404\"} 1\n\
             # HELP test_parse_error Parse log error count\n\
             # TYPE test_parse_error counter\n\
             test_parse_error 0\n\
             "
        );
    }

    #[test]
    fn test_invalid_line() {
        let (registry, metrics_collection) = metrics!();
        let line = "error";

        process_line(line, &metrics_collection, &enabled_metrics!());

        assert_eq!(
            gather_metrics!(registry),
            "# HELP test_parse_error Parse log error count\n\
             # TYPE test_parse_error counter\n\
             test_parse_error 1\n\
             "
        );
    }
}
