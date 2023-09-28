# rookie
Load cookie from your web browsers into rust

# Install
```
cargo add rookie
```

# Usage
```
use rookie::{self, Cookie};

fn main() {
    let cookies: Vec<Cookie> = rookie::chrome().unwrap();
    println!("{:?}", cookies);
}
```