mod guard;
mod init_helpers;
mod metrics;
mod parser;
mod reader;

use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer};
use actix_web_prom::PrometheusMetrics;
use clap::Clap;
use dotenv::dotenv;
use std::{io, thread};

use crate::guard::PanicGuard;
use crate::reader::{attach_file, EnabledMetric};

#[get("/health")]
fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Nginx to Prometheus basic metrics exporter
#[derive(Clap, Clone)]
#[clap(version = "0.1")]
struct Opts {
    /// Access log file to attach
    access_log: String,

    /// Prometheus namespace to prefix metrics with
    #[clap(short, long, default_value = "nginx")]
    namespace: String,

    /// Bind server to this address and port
    #[clap(short, long, default_value = "0.0.0.0:5000")]
    address: String,

    /// Enable http status code simple counter
    #[clap(long)]
    metric_status_short: bool,

    /// Enable http status code details counter
    #[clap(long)]
    metric_status_details: bool,

    /// Enable http response size counter
    #[clap(long)]
    metric_response_size: bool,
}

impl From<Opts> for EnabledMetric {
    fn from(opts: Opts) -> Self {
        EnabledMetric {
            response_status_short: opts.metric_status_short,
            response_status_full: opts.metric_status_details,
            response_size: opts.metric_response_size,
        }
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let opts = Opts::parse();
    let enabled: EnabledMetric = opts.clone().into();

    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);
    let metrics = metrics::Metrics::new(&opts.namespace);
    metrics.register(&prometheus.registry);

    init_helpers::fill_counter(&metrics.http_response_total);

    let access_log = opts.access_log.clone();

    thread::Builder::new()
        .name("parser".to_owned())
        .spawn(move || {
            let _guard = PanicGuard::new();
            attach_file(&access_log, metrics, enabled).expect("Parser panic");
        })?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(prometheus.clone())
            .service(health)
    })
    .bind(&opts.address)?
    .run()
    .await
}
