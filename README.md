# HelloWorld service

Simple HTTP service in Rust 🦀 with [axum](https://github.com/tokio-rs/axum).

## Build

```console
cargo build --release
```

## Run

```console
cargo run  # or ./target/release/helloworld
```

## Setup as a `Supervisor` service

Copy the binary `target/release/helloworld` to the remote machine folder according to [cosmian_helloworld.ini](./resources/supervisor/cosmian_helloworld.ini) statement (ie: `/usr/sbin/cosmian_helloworld`).

Copy the [cosmian_helloworld.ini](./resources/supervisor/cosmian_helloworld.ini) config file as `/etc/supervisord.d/cosmian_helloworld.ini` in the remote machine.

Run:

```console
supervisorctl reload
supervisorctl start cosmian_helloworld
supervisorctl status cosmian_helloworld
```
