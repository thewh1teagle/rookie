# Build rookie

## Prerequisites

[Rust](https://www.rust-lang.org/tools/install)

## Linux setup

```console
sudo apt-get install -y python3-dev
```

## rookie-rs

```console
git clone https://github.com/thewh1teagle/rookie
cd rookie-rs
cargo build
```

## cli

````console
git clone https://github.com/thewh1teagle/rookie
cd cli
cargo build --release
```

## Python Bindings

Using [maturin](https://pyo3.rs/v0.21.2/#usage)

```console
git clone https://github.com/thewh1teagle/rookie
cd bindings/python
python3 -m venv venv
source venv/bin/activate
# Install dependencies + build + install
# May take sometime on first use
pip3 install .
````

## Node Bindings

```console
bun install -g @napi-rs/cli
cd bindings/python
napi build
```
