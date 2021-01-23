FROM rust:1.49 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM ubuntu:20.04
COPY --from=builder /usr/local/cargo/bin/ngx-prom /usr/local/bin/ngx-prom
CMD ["ngx-prom"]

