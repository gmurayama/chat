# Webservice Template in Rust

Template for web services in Rust using [actix-web](https://actix.rs). It provide a basic features with useful features pre-configured from the get-go.

Based on [Zero To Production in Rust](https://github.com/LukeMathWalker/zero-to-production) by Luca Palmieri.

## Overview

### Features

- Handle [configuration](https://github.com/mehcode/config-rs) on the application using environment variables.
- By default, it has a middleware that timeout a request that takes too long
- Emit traces using the [OpenTelemetry](https://github.com/open-telemetry/opentelemetry-rust) framework any OTel Collector (such as Jaeger).
- Use [Prometheus](https://github.com/prometheus/client_rust) to send metrics.
- Dockerfile with multi-arch build

### Structure

```
webservice-template-rust/
├─ application/           # business layer
├─ gateways/              # entry points (all the binaries)
│  ├─ api/
│  │  ├─ routes/          # functions that are going to be registered on server.rs
│  │  ├─ main.rs
│  │  ├─ settings.rs      # all settings used in the API (telemetry, metrics, host:port, etc.)
│  ├─ .../
├─ infrastructure/        # ports/adapters (i.e. implementation of abstractions used in gateways and application)
├─ docker-compose.yaml    # prometheus, grafana and jaeger services (development purposes)
├─ prometheus.yaml        # basic config for prometheus
```

## Getting Started

1. Customize the project for your needs:
    - Add new routes following the example on [routes/](gateways/api/src/routes/reply.rs)
    - Configure environment variables or settings in [settings.rs](gateways/api/src/settings.rs)

2. Run the project:

```bash
$ docker-compose up -d
$ cargo run -p api
```

Access your web service at http://localhost:7000 (by default).

## Build

Requires [docker-buildx](https://github.com/docker/buildx) plugin. More details on Docker's guide "[Multi-platform builds](https://github.com/docker/buildx)"

```
$ make docker-build/multi-arch TAG=1.0.0 # build docker image with x86 and arm64 support
                                         # alternatively, run `make docker-build/arm64` or `make docker-build/amd64` for single arch image
```
