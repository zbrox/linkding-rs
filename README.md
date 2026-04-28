# linkding-rs

[![Crates.io](https://img.shields.io/crates/v/linkding-rs.svg)](https://crates.io/crates/linkding-rs)
[![Docs.rs](https://docs.rs/linkding-rs/badge.svg)](https://docs.rs/linkding-rs)
[![Build](https://github.com/zbrox/linkding-rs/actions/workflows/build.yml/badge.svg)](https://github.com/zbrox/linkding-rs/actions/workflows/build.yml)

A Rust client for the [linkding](https://linkding.link/) API with both sync and async support, plus cross platform bindings.

Tested with linkding v1.36.0.

## Cargo features

The crate ships two clients and two TLS backends. By default all are enabled.

| Feature      | Default | What it gives you                                        |
| ------------ | :-----: | -------------------------------------------------------- |
| `blocking`   |   yes   | `LinkDingClient` — synchronous, built on blocking reqwest |
| `async`      |   yes   | `LinkDingAsyncClient` — async, built on tokio + reqwest   |
| `rustls-tls` |   yes   | TLS via rustls                                           |
| `native-tls` |   no    | TLS via the system's native TLS stack                    |
| `ffi`        |   no    | UniFFI scaffolding for the sync client (mobile bindings) |

At least one TLS backend must be enabled. Disable defaults to pick your own:

```toml
linkding-rs = { version = "0.3", default-features = false, features = ["async", "rustls-tls"] }
```

## Sync usage

```rust
use linkding::{LinkDingClient, ListBookmarksArgs};

let client = LinkDingClient::new("https://linkding.local:9090", "YOUR_API_TOKEN");
let bookmarks = client.list_bookmarks(ListBookmarksArgs::default())?;
```

## Async usage

```rust
use linkding::{LinkDingAsyncClient, ListBookmarksArgs};

#[tokio::main]
async fn main() -> Result<(), linkding::LinkDingError> {
    let client = LinkDingAsyncClient::new("https://linkding.local:9090", "YOUR_API_TOKEN");
    let bookmarks = client.list_bookmarks(ListBookmarksArgs::default()).await?;
    Ok(())
}
```

## Cross platform

There are [Uniffi](https://mozilla.github.io/uniffi-rs/latest/) bindings so you can use this for making Android or iOS apps. The bindings expose the synchronous client only; mobile consumers should call from a background thread.

The CI is building a Swift package that you can download from the releases page.
