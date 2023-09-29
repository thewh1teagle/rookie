# rookie
Load cookies from your web browsers into rust

# Install
```shell
cargo add rookie
```

# Usage
```rust
use rookie::{self, Cookie};

fn main() {
    let domains = Some(vec!["google.com"]); // set to None to get all
    let cookies: Vec<Cookie> = rookie::chrome(domains).unwrap();
    println!("{:?}", cookies);
}
```
