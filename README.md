# HelloWorld service

Simple HTTP service in Rust 🦀 with [axum](https://github.com/tokio-rs/axum).

## Build

```console
cargo build --release
```

Feature `confidential` allows to add `/report` endpoint for Confidential Containers on AKS:

```console
cargo build --release --features="confidential"
```

## Run

```console
cargo run  # or ./target/release/hellworld
```
