FROM rust:1.88-alpine AS builder
RUN apk add --no-cache musl-dev
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

COPY src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

# Copy the CA certificates from Alpine
# This is necessary to ensure the application can make HTTPS requests
# without failing due to missing certificates.
COPY --from=alpine:3.20 /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-https-check /rust-https-check

ENTRYPOINT ["/rust-https-check"]