FROM rust:1.49 AS builder
WORKDIR /usr/src/app
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install --yes musl-tools
COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN strip /usr/local/cargo/bin/ngx-prom

FROM scratch
COPY --from=builder /usr/local/cargo/bin/ngx-prom .
CMD ["./ngx-prom"]
