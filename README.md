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

- Not so much features

## DONE and TODO

- [x] Nginx logs parsing + attach to logs
- [x] Expose metrics
- [x] Guard on parser thread panic
- [ ] Docker image
- [ ] Docs
