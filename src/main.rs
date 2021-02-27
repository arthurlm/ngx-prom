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

#[get("/health")]
fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Nginx to Prometheus basic metrics exporter
#[derive(Clap)]
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
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let opts = Opts::parse();

    /*
    let server_addr = matches.value_of("address").expect("Cannot read arg");
    */

    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);
    let metrics = metrics::Metrics::new(&opts.namespace);
    metrics.register(&prometheus.registry);

    init_helpers::fill_counter(&metrics.http_response_total);

    let access_log = opts.access_log.clone();

    thread::Builder::new()
        .name("parser".to_owned())
        .spawn(move || {
            let _guard = PanicGuard::new();
            crate::reader::attach_file(&access_log, metrics).expect("Parser panic");
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
