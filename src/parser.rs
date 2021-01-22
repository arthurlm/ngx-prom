use chrono::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::net::{self, IpAddr};
use std::num::ParseIntError;
use std::str::FromStr;

static RE_ROW_EXPR: &str = r#"^(?P<remote_addr>\S+) - (?P<remote_user>.*) \[(?P<time_local>.+)\] "(?P<request_method>\S+) (?P<request_path>\S+) (?P<request_protocol>\S+)" (?P<response_status>\d\d\d) (?P<response_body_bytes_sent>\d+) "(?P<http_referer>[^"]+)" "(?P<http_user_agent>[^"]+)""#;
static NGINX_LOCAL_TIME_FMT: &str = r"%d/%b/%Y:%H:%M:%S %z"; // 22/Jan/2021:17:24:17 +0000

lazy_static! {
    static ref RE_ROW: Regex = Regex::new(RE_ROW_EXPR).unwrap();
}

#[derive(Debug, PartialEq)]
pub struct LogRow {
    pub remote_addr: IpAddr,
    pub remote_user: String,
    pub time_local: DateTime<FixedOffset>,
    pub request_method: String,
    pub request_path: String,
    pub request_protocol: String,
    pub response_status: u16,
    pub response_body_bytes_sent: u64,
    pub http_referer: String,
    pub http_user_agent: String,
}

#[derive(Debug, PartialEq)]
pub enum ParseLogError {
    NoMatch,
    InvalidIpAddr(net::AddrParseError),
    InvalidTime,
    InvalidStatusCode(u16),
    InvalidNumber(ParseIntError),
}

impl FromStr for LogRow {
    type Err = ParseLogError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        log::debug!("Parsing: {:?}", value);

        let result = RE_ROW.captures(&value).ok_or(ParseLogError::NoMatch)?;

        let response_status = result["response_status"]
            .parse()
            .map_err(ParseLogError::InvalidNumber)?;

        if response_status >= 600 || response_status < 100 {
            return Err(ParseLogError::InvalidStatusCode(response_status));
        }

        Ok(LogRow {
            remote_addr: IpAddr::from_str(&result["remote_addr"])
                .map_err(ParseLogError::InvalidIpAddr)?,
            remote_user: result["remote_user"].to_owned(),
            time_local: DateTime::parse_from_str(&result["time_local"], NGINX_LOCAL_TIME_FMT)
                .map_err(|_e| ParseLogError::InvalidTime)?,
            request_method: result["request_method"].to_owned(),
            request_path: result["request_path"].to_owned(),
            request_protocol: result["request_protocol"].to_owned(),
            response_status,
            response_body_bytes_sent: result["response_body_bytes_sent"]
                .parse()
                .map_err(ParseLogError::InvalidNumber)?,
            http_referer: result["http_referer"].to_owned(),
            http_user_agent: result["http_user_agent"].to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    macro_rules! invalid_ip {
        () => {
            "".parse::<IpAddr>().err().unwrap()
        };
    }

    #[test]
    fn test_empty() {
        assert_eq!(LogRow::from_str(""), Err(ParseLogError::NoMatch));
    }

    #[test]
    fn test_invalid_ip() {
        assert_eq!(
            LogRow::from_str(
                r#"999.999.999.999 - - [22/Jan/2021:17:24:16 +0000] "GET / HTTP/1.1" 304 0 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:84.0) Gecko/20100101 Firefox/84.0" 0.2"#
            ),
            Err(ParseLogError::InvalidIpAddr(invalid_ip!()))
        );
        assert_eq!(
            LogRow::from_str(
                r#"256.256.256.256 - - [22/Jan/2021:17:24:16 +0000] "GET / HTTP/1.1" 304 0 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:84.0) Gecko/20100101 Firefox/84.0" 0.2"#
            ),
            Err(ParseLogError::InvalidIpAddr(invalid_ip!()))
        );
    }

    #[test]
    fn test_invalid_time() {
        assert_eq!(
            LogRow::from_str(
                r#"192.168.1.129 - - [31/Feb/2021:17:24:16 +0000] "GET / HTTP/1.1" 304 0 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:84.0) Gecko/20100101 Firefox/84.0" 0.2"#
            ),
            Err(ParseLogError::InvalidTime)
        );
    }

    #[test]
    fn test_invalid_status() {
        assert_eq!(
            LogRow::from_str(
                r#"192.168.1.129 - - [10/Mar/2021:17:24:16 +0000] "GET / HTTP/1.1" 631 0 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:84.0) Gecko/20100101 Firefox/84.0" 0.2"#
            ),
            Err(ParseLogError::InvalidStatusCode(631))
        );
        assert_eq!(
            LogRow::from_str(
                r#"192.168.1.129 - - [10/Mar/2021:17:24:16 +0000] "GET / HTTP/1.1" 031 0 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:84.0) Gecko/20100101 Firefox/84.0" 0.2"#
            ),
            Err(ParseLogError::InvalidStatusCode(31))
        );
    }

    #[test]
    fn test_valid_with_extra() {
        assert_eq!(
            LogRow::from_str(
                r#"192.168.1.84 - - [22/Jan/2021:17:24:16 +0000] "GET / HTTP/1.1" 304 0 "-" "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:84.0) Gecko/20100101 Firefox/84.0" 0.24 this is a test"#
            ),
            Ok(LogRow {
                remote_addr: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 84)),
                remote_user: "-".to_owned(),
                time_local: DateTime::parse_from_rfc3339("2021-01-22T17:24:16+00:00").unwrap(),
                request_method: "GET".to_owned(),
                request_path: "/".to_owned(),
                request_protocol: "HTTP/1.1".to_owned(),
                response_status: 304,
                response_body_bytes_sent: 0,
                http_referer: "-".to_owned(),
                http_user_agent:
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:84.0) Gecko/20100101 Firefox/84.0"
                        .to_owned()
            })
        );
    }
}
