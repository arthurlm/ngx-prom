use lazy_static::lazy_static;
use prometheus::IntCounterVec;

lazy_static! {
    static ref HTTP_CODES: Vec<usize> = {
        vec![
            100, 101, 102, 200, 201, 202, 203, 204, 205, 206, 207, 208, 226, 300, 301, 302, 303,
            304, 305, 307, 308, 400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412,
            413, 414, 415, 416, 417, 418, 421, 422, 423, 424, 426, 428, 429, 431, 444, 451, 499,
            500, 501, 502, 503, 504, 505, 506, 507, 508, 510, 511, 599,
        ]
    };
}

pub fn fill_counter(metrics: &IntCounterVec) {
    for status in HTTP_CODES.iter() {
        let status = format!("{}", status);

        metrics.with_label_values(&[&status]).reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus::{Encoder, Opts, Registry, TextEncoder};

    macro_rules! collect {
        ($metric:expr) => {{
            let registry = Registry::new();
            registry.register(Box::new($metric.clone())).unwrap();

            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            encoder.encode(&registry.gather(), &mut buffer).unwrap();
            String::from_utf8(buffer).unwrap()
        }};
    }

    #[test]
    fn test_init_with_0() {
        let metric = IntCounterVec::new(Opts::new("test", "Test metric"), &["status"]).unwrap();
        assert_eq!(collect!(metric), "");

        fill_counter(&metric);
        assert_ne!(collect!(metric), "");
    }
}
