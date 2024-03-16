# Rookie Rust Docs

## Install

```console
cargo add rookie
```

## Basic Usage

```rust
use rookie;

fn main() {
    let cookies = rookie::chrome(None).unwrap();
    println!("{cookies:?}");
}
```

## Logging

Logging level can be controlled by changing `RUST_LOG` ENV variable

```console
RUST_LOG=trace cargo run
```
