mod guard;
mod metrics;
mod parser;
mod reader;

use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer};
use actix_web_prom::PrometheusMetrics;
use dotenv::dotenv;
use std::{io, thread};

use crate::guard::PanicGuard;

#[get("/health")]
fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let matches = clap::App::new("Nginx to Prometheus")
        .version("0.1")
        .about("Nginx to Prometheus basic metrics exporter")
        .arg(
            clap::Arg::with_name("access_log")
                .help("Access log file to attach")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("namespace")
                .short("n")
                .long("namespace")
                .help("Prometheus namespace to prefix metrics with")
                .required(false)
                .takes_value(true)
                .default_value("nginx"),
        )
        .arg(
            clap::Arg::with_name("address")
                .short("a")
                .long("address")
                .help("Bind server to this address and port")
                .required(false)
                .takes_value(true)
                .default_value("0.0.0.0:5000"),
        )
        .get_matches();

    let filename = matches
        .value_of("access_log")
        .expect("Cannot read arg")
        .to_owned();
    let namespace = matches.value_of("namespace").expect("Cannot read arg");
    let server_addr = matches.value_of("address").expect("Cannot read arg");

    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);
    let metrics = metrics::Metrics::new(namespace);
    metrics.register(&prometheus.registry);

    thread::Builder::new()
        .name("parser".to_owned())
        .spawn(move || {
            let _guard = PanicGuard::new();
            crate::reader::attach_file(filename, metrics).expect("Parser panic");
        })?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(prometheus.clone())
            .service(health)
    })
    .bind(server_addr)?
    .run()
    .await
}
