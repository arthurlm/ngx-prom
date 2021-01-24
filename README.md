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

## Available options

```text
Nginx to Prometheus 0.1
Nginx to Prometheus basic metrics exporter

USAGE:
    ngx-prom [OPTIONS] <access_log>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --address <address>        Bind server to this address and port [default: 0.0.0.0:5000]
    -n, --namespace <namespace>    Prometheus namespace to prefix metrics with [default: nginx]

ARGS:
    <access_log>    Access log file to attach
```

## DONE and TODO

- [x] Nginx logs parsing + attach to logs
- [x] Expose metrics
- [x] Guard on parser thread panic
- [x] Docker image
- [ ] Docs
