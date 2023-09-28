# rookie
Load cookie from your web browsers into rust

# Install
```shell
cargo add rookie
```

# Usage
```rust
use rookie::{self, Cookie};

fn main() {
    let cookies: Vec<Cookie> = rookie::chrome().unwrap();
    println!("{:?}", cookies);
}
```
