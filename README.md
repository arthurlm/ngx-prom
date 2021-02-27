# Nginx to prometheus metrics exporter

Yet another nginx to prometheus metrics exporter.

## Why using it ?

Pros:

- App is dead simple (less than 500 lines of code with unit tests)
- Based on logs regex parse
- Fast and ultra low memory footprint
- Do not require Nginx Plus
- Do not need to rebuild nginx with custom extension
- Works with all version of nginx

Cons:

- Not so many features
- Not so many metrics for now

## How to use it

Using docker:

```sh
docker run --rm \
    --name ngx-prom \
    -e RUST_LOG=info \
    -v /var/log/nginx:/log \
    arthurlm/ngx-prom:latest ngx-prom /log/access.log
```

Using shell:

```sh
ngx-prom /var/log/nginx/access.log
```

## Served metrics example

Example of response from `/metrics` route:

```text
# HELP nginx_http_response_body_size_total Size of HTTP request per request info
# TYPE nginx_http_response_body_size_total counter
nginx_http_response_body_size_total{method="GET",path="/",protocol="HTTP/1.1"} 274
nginx_http_response_body_size_total{method="GET",path="/favicon.ico",protocol="HTTP/1.1"} 134

# HELP nginx_http_response_code_total Count of HTTP request per request info and response code
# TYPE nginx_http_response_code_total counter
nginx_http_response_code_total{method="GET",path="/",protocol="HTTP/1.1",status="200"} 1
nginx_http_response_code_total{method="GET",path="/",protocol="HTTP/1.1",status="304"} 5
nginx_http_response_code_total{method="GET",path="/favicon.ico",protocol="HTTP/1.1",status="404"} 1

# HELP nginx_http_response_total Number of HTTP request by status code
# TYPE nginx_http_response_total counter
nginx_http_response_total{status="200"} 1
nginx_http_response_total{status="304"} 5
nginx_http_response_total{status="404"} 1

# HELP nginx_parse_error Parse log error count
# TYPE nginx_parse_error counter
nginx_parse_error 0
```

## Available options

```text
ngx-prom 0.1
Nginx to Prometheus basic metrics exporter

USAGE:
    ngx-prom [FLAGS] [OPTIONS] <access-log>

ARGS:
    <access-log>    Access log file to attach

FLAGS:
    -h, --help                     Prints help information
        --metric-response-size     Enable http response size counter
        --metric-status-details    Enable http status code details counter
        --metric-status-short      Enable http status code simple counter
    -V, --version                  Prints version information

OPTIONS:
    -a, --address <address>        Bind server to this address and port [default: 0.0.0.0:5000]
    -n, --namespace <namespace>    Prometheus namespace to prefix metrics with [default: nginx]
```

## DONE and TODO

- [x] Nginx logs parsing + attach to logs
- [x] Expose metrics
- [x] Guard on parser thread panic
- [x] Docker image
- [x] Docs
