use prometheus::{IntCounterVec, Opts, Registry};

#[derive(Debug)]
pub struct Metrics {
    pub http_response_code_total: IntCounterVec,
    pub http_response_body_size_total: IntCounterVec,
}

impl Metrics {
    pub fn new(namespace: &str) -> Self {
        Self {
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
        }
    }

    pub fn register(&self, registry: &Registry) {
        registry
            .register(Box::new(self.http_response_code_total.clone()))
            .unwrap();
        registry
            .register(Box::new(self.http_response_body_size_total.clone()))
            .unwrap();
    }
}
