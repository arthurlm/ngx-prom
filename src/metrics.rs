use prometheus::{IntCounter, IntCounterVec, Opts, Registry};

#[derive(Debug)]
pub struct Metrics {
    pub http_response_total: IntCounterVec,
    pub http_response_code_total: IntCounterVec,
    pub http_response_body_size_total: IntCounterVec,
    pub parse_error: IntCounter,
}

impl Metrics {
    pub fn new(namespace: &str) -> Self {
        Self {
            http_response_total: IntCounterVec::new(
                Opts::new(
                    "http_response_total",
                    "Number of HTTP request by status code",
                )
                .namespace(namespace),
                &["status"],
            )
            .unwrap(),
            http_response_code_total: IntCounterVec::new(
                Opts::new(
                    "http_response_code_total",
                    "Count of HTTP request per request info and response code",
                )
                .namespace(namespace),
                &["method", "path", "protocol", "status"],
            )
            .unwrap(),
            http_response_body_size_total: IntCounterVec::new(
                Opts::new(
                    "http_response_body_size_total",
                    "Size of HTTP request per request info",
                )
                .namespace(namespace),
                &["method", "path", "protocol"],
            )
            .unwrap(),
            parse_error: IntCounter::with_opts(
                Opts::new("parse_error", "Parse log error count").namespace(namespace),
            )
            .unwrap(),
        }
    }

    pub fn register(&self, registry: &Registry) {
        macro_rules! register_metric {
            ($metric:expr) => {
                registry.register(Box::new($metric.clone())).unwrap()
            };
        }

        register_metric!(self.http_response_total);
        register_metric!(self.http_response_code_total);
        register_metric!(self.http_response_body_size_total);
        register_metric!(self.parse_error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_do_not_panic() {
        let registry = Registry::new();
        let metric = Metrics::new("test_ns");
        metric.register(&registry);
    }
}
