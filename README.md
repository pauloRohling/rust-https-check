# rust-https-check

Minimal Rust HTTPS client to reproduce the common missing CA certificates problem in `scratch`-based Docker images.

This project contains:

- **broken.Dockerfile** → Multi-stage build **without** CA certificates (HTTPS requests fail).
- **Dockerfile** → Multi-stage build **with** CA certificates (HTTPS requests succeed).
- **Makefile** → Helper commands to build and run both versions quickly.

## How it works

- The Rust application uses [`reqwest`](https://docs.rs/reqwest) with `rustls-tls-native-roots`, which relies on the system's certificate store.
- In a `scratch` image, `/etc/ssl/certs` is missing by default, causing HTTPS requests to fail.
- Adding the `ca-certificates` file fixes the problem.

## Requirements

- [Docker](https://docs.docker.com/get-docker/)  
- [Make](https://www.gnu.org/software/make/) (optional, but recommended)

## Quick usage with Makefile

### Broken build (no CA certificates)
```bash
make build-broken
make run-broken
```

**Expected output**: A certificate verification error due to missing root CAs.

### Fixed build (with CA certificates)
```bash
make build
make run
```

**Expected output**: `Status: 200 OK` with the number of bytes from the response body.